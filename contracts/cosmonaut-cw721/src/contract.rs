#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::execute::{
    execute_approve, execute_approve_all, execute_burn, execute_decrease_health,
    execute_load_freight, execute_mint, execute_revoke, execute_revoke_all, execute_send_nft,
    execute_set_minter, execute_transfer_nft, execute_unload_freight,
};
use crate::msg::ExecuteMsg;
use crate::query::{
    query_all_nft_info, query_approved_for_all, query_contract_info, query_nft_info,
    query_num_tokens, query_owner_of, query_tokens,
};
use crate::state::{CosmonautContract, Extension};
use cw2::set_contract_version;
use cw721_base::{InstantiateMsg, QueryMsg};

const CONTRACT_NAME: &str = "crates.io:cosmonaut-cw721";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CosmonautContract::default().instantiate(deps, env, info.clone(), msg)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg<Extension>,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => execute_transfer_nft(deps, env, token_id, info.sender, recipient),
        ExecuteMsg::Mint(mint_msg) => execute_mint(deps, env, info, mint_msg),
        ExecuteMsg::SendNft {
            contract,
            token_id,
            msg,
        } => execute_send_nft(deps, env, &token_id, info.sender, contract, msg),
        ExecuteMsg::Approve {
            spender,
            token_id,
            expires,
        } => execute_approve(deps, env, info, spender, token_id, expires),
        ExecuteMsg::Revoke { spender, token_id } => {
            execute_revoke(deps, env, info, spender, token_id)
        }
        ExecuteMsg::Burn { token_id } => execute_burn(deps, env, info, token_id),
        ExecuteMsg::ApproveAll { operator, expires } => {
            execute_approve_all(deps, env, info, operator, expires)
        }
        ExecuteMsg::RevokeAll { operator } => execute_revoke_all(deps, env, info, operator),
        ExecuteMsg::SetMinter { minter } => execute_set_minter(deps, info, minter),
        // msg to load cw20 token data on nft
        ExecuteMsg::LoadFreight {
            token_id,
            denom,
            amount,
            unit_weight,
        } => execute_load_freight(deps, token_id, denom, amount, unit_weight),
        // msg to unload cw20 token data on nft
        ExecuteMsg::UnloadFreight {
            token_id,
            denom,
            amount,
        } => execute_unload_freight(deps, token_id, denom, amount),
        // msg to decrease health when playing games
        ExecuteMsg::DecreaseHealth { token_id, value } => {
            execute_decrease_health(deps, info, env, token_id, value)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => to_binary(&query_owner_of(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => to_binary(&query_approved_for_all(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::NftInfo { token_id } => to_binary(&query_nft_info(deps, token_id)?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_binary(&query_all_nft_info(
            deps,
            env,
            token_id,
            include_expired.unwrap_or_default(),
        )?),
        QueryMsg::NumTokens {} => to_binary(&query_num_tokens(deps)?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_binary(&query_tokens(deps, owner, start_after, limit)?),
        QueryMsg::ContractInfo {} => to_binary(&query_contract_info(deps)?),

        _ => StdResult::Ok(Default::default()),
    }
}
