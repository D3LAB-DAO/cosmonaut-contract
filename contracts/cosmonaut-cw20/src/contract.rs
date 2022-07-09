#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, MinterResponse, QueryMsg};
use crate::state::{TokenInfo, BALANCES, TOKEN_INFO};
use crate::{execute, query};

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
        total_supply: Some(Uint128::new(msg.total_supply.unwrap_or(0))),
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
            .unwrap_or_default()
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
    match msg {
        ExecuteMsg::Transfer { recipient, amount } => {
            execute::transfer(deps, info.sender, recipient, amount)
        }
        ExecuteMsg::Mint { recipient, amount } => {
            execute::mint(deps, info.sender, recipient, amount)
        }
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => execute::send(deps, info.sender, contract, amount, msg),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute::increase_allowance(deps, info.sender, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute::decrease_allowance(deps, info.sender, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute::transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::Burn { amount } => execute::burn(deps, env, info, amount),
        ExecuteMsg::BurnFrom { owner, amount } => {
            execute::burn_from(deps, env, info, owner, amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => query::balance(deps, address),
        QueryMsg::TokenInfo {} => query::token_info(deps),
        QueryMsg::MintInfo {} => query::mint_info(deps),
        QueryMsg::Allowance { owner, spender } => {
            query::allowance(deps, owner, spender)
        }
    }
}
