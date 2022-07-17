#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use std::convert::TryInto;

use crate::execute::{execute_update_minter, set_token_extension};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw20_base::allowances::{
    execute_burn_from, execute_decrease_allowance, execute_increase_allowance, execute_send_from,
    execute_transfer_from,
};
use cw20_base::contract::{
    execute_burn, execute_mint, execute_send, execute_transfer, execute_update_marketing,
    execute_upload_logo, instantiate as cw20_instantiate, query as cw20_query,
};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw20_base::ContractError;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// TODO: q1) Declare TokenExtension struct witch implements Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema
// TokenExtension has unit_weight: Uint128 as public field


pub const TOKEN_EXTENSION: Item<TokenExtension> = Item::new("token_extension");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    if let Some(token_extension) = msg.token_extension {
        TOKEN_EXTENSION.save(deps.storage, &token_extension)?;
    }
    let cw20_instantiate_msg = Cw20InstantiateMsg {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        initial_balances: msg.initial_balances,
        mint: msg.mint,
        marketing: msg.marketing,
    };

    cw20_instantiate(deps, _env, _info, cw20_instantiate_msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // TODO: q2) Route ExecuteMsg::Transfer, ExecuteMsg::Burn
        // call execute_transfer for Transfer,
        // call execute_burn for Burn

        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => execute_send(deps, env, info, contract, amount, msg),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => execute_decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => execute_transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => execute_send_from(deps, env, info, owner, contract, amount, msg),
        ExecuteMsg::BurnFrom { owner, amount } => execute_burn_from(deps, env, info, owner, amount),
        ExecuteMsg::Mint { recipient, amount } => execute_mint(deps, env, info, recipient, amount),
        ExecuteMsg::UpdateMarketing {
            project,
            description,
            marketing,
        } => execute_update_marketing(deps, env, info, project, description, marketing),
        ExecuteMsg::UpdateMinter { new_minter } => execute_update_minter(deps, info, new_minter),
        ExecuteMsg::UploadLogo(logo) => execute_upload_logo(deps, env, info, logo),
        ExecuteMsg::SetTokenExtension { unit_weight } => set_token_extension(deps, unit_weight),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenExtension {} => to_binary(&query::token_extension(deps)?),
        _ => cw20_query(deps, env, msg.try_into()?),
    }
}
