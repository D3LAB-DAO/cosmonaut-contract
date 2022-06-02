use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{FreightContractInfo, CONFIG};
use cosmonaut_cw20::msg as cosmonaut_cw20_msg;
use cosmonaut_cw721::msg as cosmonaut_cw721_msg;
use cosmonaut_cw721::state::{Extension, Metadata};
use cosmwasm_std::{
    coin, to_binary, Addr, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128, WasmMsg,
};
use cw721::{Cw721QueryMsg, NftInfoResponse, OwnerOfResponse};
use cw721_base::{MintMsg, QueryMsg};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const MAX_TOTAL_WEIGHT: u128 = 1000 * 100000;

pub fn execute_mint_to_cw721_contract(
    deps: DepsMut,
    _info: MessageInfo,
    mint_msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mint_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config
            .spaceship_cw721_contract
            .addr
            .as_ref()
            .unwrap()
            .to_string(),
        msg: to_binary(&ExecuteMsg::Mint(mint_msg))?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_message(mint_msg_wrap))
}

pub fn execute_buy_spaceship(
    deps: DepsMut,
    info: MessageInfo,
    nft_id: String,
    _original_owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let nft_info: NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
        config.spaceship_cw721_contract.addr.as_ref().unwrap(),
        &Cw721QueryMsg::NftInfo {
            token_id: nft_id.clone(),
        },
    )?;

    let money_token_info: cosmonaut_cw20_msg::TokenInfoResponse = deps.querier.query_wasm_smart(
        config.money_cw20_contract.addr.as_ref().unwrap(),
        &cosmonaut_cw20::msg::QueryMsg::TokenInfo {},
    )?;

    if money_token_info.symbol != nft_info.extension.unit_denom {
        return Err(ContractError::InvalidToken {});
    }

    let token_balance: cosmonaut_cw20_msg::BalanceResponse = deps.querier.query_wasm_smart(
        config.money_cw20_contract.addr.as_ref().unwrap(),
        &cosmonaut_cw20_msg::QueryMsg::Balance {
            address: info.sender.to_string(),
        },
    )?;

    if token_balance.balance.u128() < nft_info.extension.price {
        return Err(ContractError::NotEnoughToken {});
    }

    let transfer_money_msg = cosmonaut_cw20_msg::ExecuteMsg::TransferFrom {
        owner: info.sender.to_string(),
        recipient: config
            .money_cw20_contract
            .addr
            .as_ref()
            .unwrap()
            .to_string(),
        amount: Uint128::from(nft_info.extension.price),
    };

    let transfer_money_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.addr.unwrap().to_string(),
        msg: to_binary(&transfer_money_msg)?,
        funds: vec![],
    });

    let transfer_nft_msg: cosmonaut_cw721::msg::ExecuteMsg<Extension> =
        cosmonaut_cw721_msg::ExecuteMsg::TransferNft {
            recipient: info.sender.to_string(),
            token_id: nft_id,
        };

    let transfer_nft_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
        msg: to_binary(&transfer_nft_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_spaceship")
        .add_attribute("price", nft_info.extension.price.to_string())
        .add_messages([transfer_money_msg_wrap, transfer_nft_msg_wrap]))
}

pub fn execute_set_minter_to_cw721_contract(
    deps: DepsMut,
    minter: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let set_minter_msg: ExecuteMsg<Extension> = ExecuteMsg::SetMinter { minter };

    let set_minter_msg_wrapper = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
        msg: to_binary(&set_minter_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_message(set_minter_msg_wrapper))
}

pub fn execute_load_freight_to_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    denom: String,
    amount: u128,
    unit_weight: u128,
) -> Result<Response, ContractError> {
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
        contract_addr: target_contract_addr.unwrap().address,
        msg: to_binary(&cosmonaut_cw20_msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    });

    let load_freight_msg: cosmonaut_cw721_msg::ExecuteMsg<Extension> =
        cosmonaut_cw721_msg::ExecuteMsg::LoadFreight {
            token_id: token_id.clone(),
            denom: denom.clone(),
            amount,
            unit_weight,
        };

    let load_freight_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
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

pub fn execute_unload_freight_from_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    denom: String,
    amount: u128,
) -> Result<Response, ContractError> {
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
        msg: to_binary(&cosmonaut_cw20_msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    });

    let unload_freight_msg: cosmonaut_cw721_msg::ExecuteMsg<Extension> =
        cosmonaut_cw721_msg::ExecuteMsg::UnloadFreight {
            token_id: token_id.clone(),
            denom: denom.clone(),
            amount,
        };

    let unload_freight_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
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

pub fn execute_add_freight_contract(
    deps: DepsMut,
    address: String,
    denom: String,
    code_id: u64,
) -> Result<Response, ContractError> {
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
        config.spaceship_cw721_contract.addr.as_ref().unwrap(),
        &QueryMsg::OwnerOf {
            token_id: token_id.to_string(),
            include_expired: Option::from(false),
        },
    )?;

    if owner_of_query_res.owner != *sender
        && owner_of_query_res
            .approvals
            .into_iter()
            .filter(|a| a.spender == *sender)
            .count()
            == 0
    {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub fn execute_buy_money_token(
    deps: DepsMut,
    info: MessageInfo,
    amount: u128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let income_asset = info.funds;

    let atom_income = income_asset
        .into_iter()
        .find(|coin| coin.denom == "uatom")
        .unwrap_or_else(|| coin(0, "uatom"));

    if atom_income.amount.u128() < amount {
        return Err(ContractError::NotEnoughCoin {});
    }

    let mint_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config
            .money_cw20_contract
            .addr
            .as_ref()
            .unwrap()
            .to_string(),
        msg: to_binary(&cosmonaut_cw20_msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_money_token".to_string())
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("amount", amount.to_string())
        .add_message(mint_token_msg))
}

pub fn execute_buy_freight_token(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let target_contract_addr = config
        .freight_contracts
        .into_iter()
        .find(|c| c.denom == denom);

    if target_contract_addr.is_none() {
        return Err(ContractError::TokenNotFound {});
    }

    let mint_target_token_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: target_contract_addr.unwrap().address,
        msg: to_binary(&cosmonaut_cw20_msg::ExecuteMsg::Mint {
            recipient: info.sender.to_string(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    });

    let burn_money_token_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.addr.unwrap().to_string(),
        msg: to_binary(&cosmonaut_cw20_msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_freight_token")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("denom", denom)
        .add_attribute("amount", amount.to_string())
        .add_messages([mint_target_token_msg_wrap, burn_money_token_msg_wrap]))
}

pub fn execute_play_game(
    deps: DepsMut,
    env: Env,
    token_id: String,
    epoch: u64,
) -> Result<Response, ContractError> {
    let mut seed = StdRng::seed_from_u64(env.block.time.nanos());
    let config = CONFIG.load(deps.storage)?;

    let nft_info: NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
        config
            .spaceship_cw721_contract
            .addr
            .as_ref()
            .unwrap()
            .to_string(),
        &Cw721QueryMsg::NftInfo {
            token_id: token_id.clone(),
        },
    )?;

    let total_freight_weight: u128 = nft_info
        .extension
        .freight
        .iter()
        .map(|f| f.unit_weight * f.unit_weight)
        .sum();

    let mut count = 0;

    for _ in 0..epoch {
        let num: u128 = seed.gen_range(0..=(MAX_TOTAL_WEIGHT - total_freight_weight));

        if (num as f64 / MAX_TOTAL_WEIGHT as f64) < 0.5 {
            count += 1;
        }
    }

    let decrease_health_msg: cosmonaut_cw721_msg::ExecuteMsg<Extension> =
        cosmonaut_cw721_msg::ExecuteMsg::DecreaseHealth {
            token_id,
            value: count,
        };

    let decrease_health_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config
            .spaceship_cw721_contract
            .addr
            .as_ref()
            .unwrap()
            .to_string(),
        msg: to_binary(&decrease_health_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "play_game")
        .add_attribute("decrease_value", count.to_string())
        .add_message(decrease_health_msg_wrap))
}
