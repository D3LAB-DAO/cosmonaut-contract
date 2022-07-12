#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, to_binary, Uint128};
use cw2::set_contract_version;
use cw20_base::contract::{execute as cw20_execute, query as cw20_query, query_token_info};
use cw20_base::msg::{ExecuteMsg, QueryMsg};
use cw20_base::{ContractError, msg};

use crate::msg::{InstantiateMsg, MinterResponse};
use crate::query;
use crate::query::token_info;
use crate::state::{TokenInfo, BALANCES, TOKEN_INFO};

const CONTRACT_NAME: &str = "crates.io:mars-tokens";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let minter_data = match msg.mint {
        Some(mint_data) => {
            let validate_minter = deps.api.addr_validate(&mint_data.minter)?;
            MinterResponse {
                minter: String::from(validate_minter),
                cap: mint_data.cap,
            }
        }
        // if there is no minter data, address who instantiate the contract becomes the minter
        None => MinterResponse {
            minter: String::from(info.sender),
            cap: None,
        },
    };

    let token_info = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        mint: Some(minter_data.clone()),
        total_supply: msg.total_supply,
        unit_weight: msg.unit_weight,
    };

    let initial_balances = msg.initial_balances;

    for balance in initial_balances {
        // initialize balances
        BALANCES.save(
            deps.storage,
            &Addr::unchecked(balance.address),
            &balance.amount,
        )?;
        // update total supply with initial balances
        token_info
            .total_supply
            .checked_add(balance.amount)
            .map_err(StdError::overflow)?;
    }

    TOKEN_INFO.save(deps.storage, &token_info)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("minter_address", minter_data.minter)
        .add_attribute("minter_cap", minter_data.cap.unwrap_or_default()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let a = cw20_execute(deps, env, info, msg);
    println!("{:?}", a);
    a
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenInfo {} => to_binary(&query::token_info(deps)?),
        _ => cw20_query(deps, env, msg),
    }
}
