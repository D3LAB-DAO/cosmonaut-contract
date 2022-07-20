use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{FreightContractInfo, CONFIG};
use cosmonaut_cw20::contract::TokenExtension;
use cosmonaut_cw721::msg::ExecuteMsg as Cw721ExecuteMsg;
use cosmonaut_cw721::state::{Extension, Metadata};
use cosmwasm_std::{
    attr, coin, to_binary, Addr, ContractInfoResponse, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, Uint128, WasmMsg, WasmQuery,
};
use cw20::{BalanceResponse, TokenInfoResponse};
use cw721::{Cw721QueryMsg, NftInfoResponse, OwnerOfResponse};
use cw721_base::{MintMsg, QueryMsg};
use std::ops::{Add, Div, Rem};

const MAX_FREIGHT_WEIGHT: u128 = 1000 * 1000;
const FUEL_PER_GAME: u128 = 100;

pub fn mint_to_cw721_contract(
    deps: DepsMut,
    _info: MessageInfo,
    mint_msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mint_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.as_ref().to_string(),
        msg: to_binary(&ExecuteMsg::Mint(mint_msg))?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_message(mint_msg_wrap))
}

pub fn buy_spaceship(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let nft_info: NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
        config.spaceship_cw721_contract.as_ref(),
        &Cw721QueryMsg::NftInfo {
            token_id: nft_id.clone(),
        },
    )?;

    let token_balance: BalanceResponse = deps.querier.query_wasm_smart(
        config.money_cw20_contract.as_ref(),
        &cw20_base::msg::QueryMsg::Balance {
            address: info.sender.to_string(),
        },
    )?;

    if token_balance.balance < nft_info.extension.price {
        return Err(ContractError::NotEnoughToken {});
    }

    let transfer_money_msg = cw20_base::msg::ExecuteMsg::TransferFrom {
        owner: info.sender.to_string(),
        recipient: env.contract.address.to_string(),
        amount: Uint128::from(nft_info.extension.price),
    };

    let transfer_money_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.to_string(),
        msg: to_binary(&transfer_money_msg)?,
        funds: vec![],
    });

    let transfer_nft_msg: cosmonaut_cw721::msg::ExecuteMsg = Cw721ExecuteMsg::TransferNft {
        recipient: info.sender.to_string(),
        token_id: nft_id,
    };

    let transfer_nft_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.to_string(),
        msg: to_binary(&transfer_nft_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_spaceship")
        .add_attribute("price", nft_info.extension.price.to_string())
        .add_messages([transfer_money_msg_wrap, transfer_nft_msg_wrap]))
}

pub fn set_minter_to_cw721_contract(
    deps: DepsMut,
    minter: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let set_minter_msg: ExecuteMsg = ExecuteMsg::SetMinter { minter };

    let set_minter_msg_wrapper = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.to_string(),
        msg: to_binary(&set_minter_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_message(set_minter_msg_wrapper))
}

pub fn load_freight_to_nft(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let token_extension: TokenExtension = deps.as_ref().querier.query_wasm_smart(
        address.clone(),
        &cosmonaut_cw20::msg::QueryMsg::TokenExtension {},
    )?;

    let token_info: TokenInfoResponse = deps
        .as_ref()
        .querier
        .query_wasm_smart(address, &cw20_base::msg::QueryMsg::TokenInfo {})?;

    let unit_weight = token_extension.unit_weight;
    let denom = token_info.symbol;

    if amount * unit_weight > Uint128::new(MAX_FREIGHT_WEIGHT) {
        return Err(ContractError::FrightOverloaded {});
    }
    let config = CONFIG.load(deps.storage)?;
    let target_contract_addr = config
        .freight_contracts
        .into_iter()
        .find(|c| c.denom == denom);

    if target_contract_addr.is_none() {
        return Err(ContractError::TokenNotFound {});
    }

    check_is_sender_owner_of_nft(deps.as_ref(), &info.sender, &token_id)?;

    let burn_cw20_token_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: target_contract_addr
            .ok_or(ContractError::TokenNotFound {})?
            .address,
        msg: to_binary(&cw20_base::msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    let load_freight_msg = Cw721ExecuteMsg::LoadFreight {
        token_id: token_id.clone(),
        denom: denom.clone(),
        amount,
        unit_weight,
    };

    let load_freight_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.to_string(),
        msg: to_binary(&load_freight_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "load_freight")
        .add_attribute("token_id", &token_id)
        .add_attribute("denom", &denom)
        .add_attribute("amount", amount.to_string())
        .add_messages([burn_cw20_token_msg_wrap, load_freight_msg_wrap]))
}

pub fn unload_freight_from_nft(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let freight_info: TokenInfoResponse = deps
        .as_ref()
        .querier
        .query_wasm_smart(address, &cw20_base::msg::QueryMsg::TokenInfo {})?;
    let denom = freight_info.symbol;

    let config = CONFIG.load(deps.storage)?;
    let target_contract_addr = config
        .freight_contracts
        .into_iter()
        .find(|c| c.denom == denom);

    if target_contract_addr.is_none() {
        return Err(ContractError::TokenNotFound {});
    }

    check_is_sender_owner_of_nft(deps.as_ref(), &info.sender, &token_id)?;

    let mint_cw20_token_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: target_contract_addr.unwrap().address,
        msg: to_binary(&cw20_base::msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    let unload_freight_msg = Cw721ExecuteMsg::UnloadFreight {
        token_id: token_id.clone(),
        denom: denom.clone(),
        amount,
    };

    let unload_freight_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.to_string(),
        msg: to_binary(&unload_freight_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "unload_freight")
        .add_attribute("token_id", &token_id)
        .add_attribute("denom", &denom)
        .add_attribute("amount", amount.to_string())
        .add_messages([mint_cw20_token_msg_wrap, unload_freight_msg_wrap]))
}

pub fn add_freight_contract(deps: DepsMut, address: String) -> Result<Response, ContractError> {
    let contract_info: ContractInfoResponse =
        deps.as_ref()
            .querier
            .query(&QueryRequest::Wasm(WasmQuery::ContractInfo {
                contract_addr: address.clone(),
            }))?;
    let code_id = contract_info.code_id;
    let freight_info: TokenInfoResponse = deps
        .querier
        .query_wasm_smart(address.clone(), &cw20_base::msg::QueryMsg::TokenInfo {})?;

    let denom = freight_info.symbol;
    let config = CONFIG.load(deps.storage)?;

    if config
        .freight_contracts
        .into_iter()
        .any(|c| c.denom == denom || c.code_id == code_id)
    {
        return Err(ContractError::DuplicatedContract {});
    }

    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.freight_contracts.push(FreightContractInfo {
            address: address.clone(),
            denom,
            code_id,
        });
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute("action", "add_freight_contract")
        .add_attribute("addr", address))
}

fn check_is_sender_owner_of_nft(
    deps: Deps,
    sender: &Addr,
    token_id: &str,
) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let owner_of_query_res: OwnerOfResponse = deps.querier.query_wasm_smart(
        config.spaceship_cw721_contract.as_ref(),
        &QueryMsg::OwnerOf {
            token_id: token_id.to_string(),
            include_expired: Some(false),
        },
    )?;

    if !owner_of_query_res
        .approvals
        .into_iter()
        .any(|a| a.spender == *sender)
        && owner_of_query_res.owner != *sender
    {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn buy_money_token(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let income_asset = info.funds;

    let atom_income = income_asset
        .into_iter()
        .find(|coin| coin.denom == "uatom")
        .unwrap_or_else(|| coin(0, "uatom"));

    if atom_income.amount.u128() < amount.u128() {
        return Err(ContractError::NotEnoughCoin {});
    }

    let mint_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.as_ref().to_string(),
        msg: to_binary(&cw20_base::msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_money_token".to_string())
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("amount", amount.to_string())
        .add_message(mint_token_msg))
}

pub fn buy_freight_token(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let validated_token_addr = deps.api.addr_validate(&address)?;
    let freight_info: TokenInfoResponse = deps.as_ref().querier.query_wasm_smart(
        validated_token_addr.clone(),
        &cw20_base::msg::QueryMsg::TokenInfo {},
    )?;
    let denom = freight_info.symbol;

    let config = CONFIG.load(deps.storage)?;
    let target_contract_addr = config
        .freight_contracts
        .into_iter()
        .find(|c| c.denom == denom);

    if target_contract_addr.is_none() {
        return Err(ContractError::TokenNotFound {});
    }

    let burn_money_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.to_string(),
        msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    let mint_freight_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: validated_token_addr.to_string(),
        msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_freight_token")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_messages([burn_money_token_msg, mint_freight_token_msg]))
}

pub fn buy_fuel_token(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let burn_money_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.to_string(),
        msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    let mint_freight_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.fuel_cw20_contract.to_string(),
        msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attributes([
            attr("action", "buy_fuel_token"),
            attr("amount", amount.to_string()),
        ])
        .add_messages([burn_money_token_msg, mint_freight_token_msg]))
}

pub fn fuel_up(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    check_is_sender_owner_of_nft(deps.as_ref(), &info.sender, &token_id)?;

    let config = CONFIG.load(deps.storage)?;
    let burn_fuel_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.fuel_cw20_contract.to_string(),
        msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount,
        })?,
        funds: vec![],
    });

    let fuel_up_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::FuelUp {
            token_id: token_id.clone(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attributes([
            attr("action", "fuel_up"),
            attr("to", token_id),
            attr("amount", amount.to_string()),
        ])
        .add_messages([burn_fuel_token_msg, fuel_up_msg]))
}

pub fn burn_fuel(
    deps: DepsMut,
    token_id: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let burn_fuel_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::BurnFuel {
            token_id: token_id.clone(),
            amount,
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attributes([
            attr("action", "burn_fuel"),
            attr("of", token_id),
            attr("amount", amount.to_string()),
        ])
        .add_message(burn_fuel_msg))
}

fn _generate_random_number(timestamp_int_nanos: Uint128) -> Uint128 {
    // TODO: q3) return remainder of timestamp_int_nanos divided by MAX_FREIGHT_WEIGHT
}

pub fn play_game(
    deps: DepsMut,
    env: Env,
    token_id: String,
    epoch: Uint128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // TODO: q1) query NftInfo of config.spaceship_cw721_contract, save it to nft_info: NftInfoResponse<Metadata>

    // TODO: q2) sum every freight's weight in nft_info by calculating unit_weight * amount

    let mut health_decrease_value = Uint128::zero();
    let mut spaceship_speed = Uint128::zero();

    for // TODO: q4) iterate for parameter epoch with anonymous variable _ {
        let timestamp_int_nanos = Uint128::new(u128::from(env.block.time.nanos()));
        let total_health = nft_info.extension.health;
        let step = total_health.div(epoch);
        let random_number = _generate_random_number(timestamp_int_nanos);
        // Decrease spaceship speed inversely sum of freight weight
        spaceship_speed = Uint128::new(MAX_FREIGHT_WEIGHT)
            - Uint128::new(MAX_FREIGHT_WEIGHT)
                .multiply_ratio(total_freight_weight, MAX_FREIGHT_WEIGHT);
        if random_number > spaceship_speed {
            health_decrease_value = health_decrease_value.add(step)
        }
    }

    let decrease_health_msg = Cw721ExecuteMsg::DecreaseHealth {
        token_id: token_id.clone(),
        value: health_decrease_value,
    };

    let decrease_health_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.as_ref().to_string(),
        msg: to_binary(&decrease_health_msg)?,
        funds: vec![],
    });

    // TODO: q5) create msg to BurnFuel as much as FUEL_PER_GAME * epoch
    // Hint: use Cw721ExecuteMsg::BurnFuel


    let burn_fuel_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.as_ref().to_string(),
        msg: to_binary(&burn_fuel_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "play_game")
        .add_attribute("decreased_health_value", health_decrease_value.to_string())
        .add_attribute(
            "decreased_fuel_value",
            (FUEL_PER_GAME * epoch.u128()).to_string(),
        )
        .add_attribute("spaceship_speed", spaceship_speed.to_string())
        .add_messages([decrease_health_msg_wrap, burn_fuel_msg_wrap]))
}
