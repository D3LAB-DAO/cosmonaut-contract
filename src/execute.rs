use crate::msg::ExecuteMsg;
use crate::state::{CONFIG, LuggageContractInfo};
use crate::ContractError;
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, MessageInfo, Response, WasmMsg, Uint128, Addr, StdResult};
use cw20::Cw20Coin;
use cw721::{Cw721QueryMsg, NftInfoResponse};
use cw721_base::{MintMsg};
use cosmonaut_cw20::msg as cosmonaut_cw20_msg;
use cosmonaut_cw20::msg::MinterResponse;
use cosmonaut_cw20::state::TokenInfo;
use cosmonaut_cw721::msg as cosmonaut_cw721_msg;
use cosmonaut_cw721::state::{Extension, Metadata};

pub fn execute_mint_to_cw721_contract(
    deps: DepsMut,
    _info: MessageInfo,
    mint_msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mint_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.as_ref().unwrap().to_string(),
        msg: to_binary(&ExecuteMsg::Mint(mint_msg))?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_message(mint_msg_wrap)
    )
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
        &Cw721QueryMsg::NftInfo { token_id: nft_id.clone() },
    )?;

    let money_token_info: cosmonaut_cw20_msg::TokenInfoResponse =
        deps.querier.query_wasm_smart(
            config.money_cw20_contract.addr.as_ref().unwrap(),
            &cosmonaut_cw20::msg::QueryMsg::TokenInfo {},
        )?;

    if money_token_info.symbol != nft_info.extension.unit_denom {
        return Err(ContractError::InvalidToken {});
    }

    let token_balance: cosmonaut_cw20_msg::BalanceResponse = deps.querier.query_wasm_smart(
        config.money_cw20_contract.addr.as_ref().unwrap(),
        &cosmonaut_cw20_msg::QueryMsg::Balance {
            address: info.sender.to_string()
        },
    )?;

    if token_balance.balance.u128() < nft_info.extension.price {
        return Err(ContractError::NotEnoughToken {});
    }

    let transfer_money_msg = cosmonaut_cw20_msg::ExecuteMsg::TransferFrom {
        owner: info.sender.to_string(),
        recipient: config.money_cw20_contract.addr.as_ref().unwrap().to_string(),
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
            token_id: nft_id.to_string(),
        };

    let transfer_nft_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
        msg: to_binary(&transfer_nft_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "buy_spaceship")
        .add_attribute("price", nft_info.extension.price.to_string())
        .add_messages([transfer_money_msg_wrap, transfer_nft_msg_wrap])
    )
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

pub fn execute_load_luggage_to_nft(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    denom: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let burn_cw20_money_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.money_cw20_contract.addr.unwrap().to_string(),
        msg: to_binary(&cosmonaut_cw20_msg::ExecuteMsg::BurnFrom {
            owner: info.sender.to_string(),
            amount: Uint128::new(amount),
        })?,
        funds: vec![],
    });

    let load_luggage_msg: cosmonaut_cw721_msg::ExecuteMsg<Extension> =
        cosmonaut_cw721_msg::ExecuteMsg::LoadLuggage {
            token_id: token_id.clone(),
            denom: denom.clone(),
            amount,
        };


    let load_luggage_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
        msg: to_binary(&load_luggage_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "load_luggage")
        .add_attribute("token_id", &token_id)
        .add_attribute("denom", &denom).
        add_attribute("amount", amount.to_string())
        .add_messages([burn_cw20_money_msg_wrap, load_luggage_msg_wrap]))
}

pub fn execute_add_luggage_contract(
    deps: DepsMut,
    address: String,
    denom: String,
    code_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let duplicated_contracts: Vec<LuggageContractInfo> = config.luggage_contracts
        .into_iter()
        .filter(|c| c.denom == denom || c.code_id == code_id)
        .collect();

    if !duplicated_contracts.is_empty() {
        return Err(ContractError::DuplicatedContract {});
    }

    CONFIG.update(
        deps.storage,
        |mut config| -> StdResult<_> {
            config.luggage_contracts.push(LuggageContractInfo {
                address: address.clone(),
                denom,
                code_id,
            });
            Ok(config)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "add_luggage_contract")
        .add_attribute("addr", address)
    )
}

