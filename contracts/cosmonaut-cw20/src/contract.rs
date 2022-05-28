#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{
    execute_burn, execute_burn_from, execute_decrease_allowance, execute_increase_allowance,
    execute_mint, execute_send, execute_transfer, execute_transfer_from,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, MinterResponse, QueryMsg};
use crate::query::{query_allowance, query_balance, query_mint_info, query_token_info};
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
        total_supply: Option::from(Uint128::new(msg.total_supply.unwrap_or(0))),
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
            execute_transfer(deps, info.sender, recipient, amount)
        }
        ExecuteMsg::Mint { recipient, amount } => {
            execute_mint(deps, info.sender, recipient, amount)
        }
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => execute_send(deps, info.sender, contract, amount, msg),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_increase_allowance(deps, info.sender, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_decrease_allowance(deps, info.sender, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
        ExecuteMsg::BurnFrom { owner, amount } => execute_burn_from(deps, env, info, owner, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance { address } => query_balance(deps, address),
        QueryMsg::TokenInfo {} => query_token_info(deps),
        QueryMsg::MintInfo {} => query_mint_info(deps),
        QueryMsg::Allowance { owner, spender } => query_allowance(deps, owner, spender),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    unimplemented!()
}
