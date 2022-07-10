#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::execute as ExecHandler;
use crate::execute::BaseExecute;
use crate::msg::ExecuteMsg;
use crate::query as QueryHandler;
use crate::state::Extension;
use cw2::set_contract_version;
use cw721_base::{Cw721Contract, InstantiateMsg, QueryMsg};

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
    let cw721_contract = Cw721Contract::<Extension, Empty>::default();
    cw721_contract.instantiate(deps, env, info.clone(), msg)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let cosmonaut_contract = Cw721Contract::default();

    match msg {
        ExecuteMsg::SetMinter { minter } => ExecHandler::execute_set_minter(deps, info, minter),
        // msg to load cw20-tokens token data on nft
        ExecuteMsg::LoadFreight {
            token_id,
            denom,
            amount,
            unit_weight,
        } => ExecHandler::execute_load_freight(deps, token_id, denom, amount, unit_weight),
        // msg to unload cw20-tokens token data on nft
        ExecuteMsg::UnloadFreight {
            token_id,
            denom,
            amount,
        } => ExecHandler::execute_unload_freight(deps, token_id, denom, amount),
        // msg to decrease health when playing games
        ExecuteMsg::DecreaseHealth { token_id, value } => {
            ExecHandler::execute_decrease_health(deps, info, env, token_id, value)
        }
        _ => cosmonaut_contract.base_execute(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Minter {} => to_binary(&QueryHandler::query_minter(deps)?),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => to_binary(&QueryHandler::query_owner_of(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => to_binary(&QueryHandler::query_approved_for_all(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::NftInfo { token_id } => to_binary(&QueryHandler::query_nft_info(deps, token_id)?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_binary(&QueryHandler::query_all_nft_info(
            deps,
            env,
            token_id,
            include_expired.unwrap_or_default(),
        )?),
        QueryMsg::NumTokens {} => to_binary(&QueryHandler::query_num_tokens(deps)?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_binary(&QueryHandler::query_tokens(
            deps,
            owner,
            start_after,
            limit,
        )?),
        QueryMsg::ContractInfo {} => to_binary(&QueryHandler::query_contract_info(deps)?),

        _ => StdResult::Ok(Default::default()),
    }
}
