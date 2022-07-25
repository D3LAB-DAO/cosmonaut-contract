#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::execute::BaseExecute;
use crate::msg::ExecuteMsg;
use crate::state::Extension;
use crate::{execute, query};

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
    // cw721_contract is constructed with <Extension, Empty>.
    // Extension type is for IndexedMap as index.
    // We wouldn't use custom reponse, passing cosmwasm_std::Empty is appropriate.
    let cw721_contract = Cw721Contract::<Extension, Empty>::default();
    // TODO: q5) instantiate cw721_contract with instantiate method
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
        ExecuteMsg::SetMinter { minter } => execute::set_minter(deps, info, minter),
        // msg to load cw20-helper token data on nft
        ExecuteMsg::LoadFreight {
            token_id,
            denom,
            amount,
            unit_weight,
        } => execute::load_freight(deps, token_id, denom, amount, unit_weight),
        // msg to unload cw20-helper token data on nft
        ExecuteMsg::UnloadFreight {
            token_id,
            denom,
            amount,
        } => execute::unload_freight(deps, token_id, denom, amount),
        // msg to decrease health when playing games
        ExecuteMsg::DecreaseHealth { token_id, value } => {
            execute::decrease_health(deps, info, env, token_id, value)
        }
        ExecuteMsg::FuelUp { token_id, amount } => execute::fuel_up(deps, info, token_id, amount),
        ExecuteMsg::BurnFuel { token_id, amount } => {
            execute::burn_fuel(deps, info, token_id, amount)
        }
        _ => cosmonaut_contract.base_execute(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Minter {} => to_binary(&query::minter(deps)?),
        QueryMsg::OwnerOf {
            token_id,
            include_expired,
        } => to_binary(&query::owner_of(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::Approval {
            token_id,
            spender,
            include_expired,
        } => to_binary(&query::approval(
            deps,
            env,
            token_id,
            spender,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::Approvals {
            token_id,
            include_expired,
        } => to_binary(&query::approved_for_all(
            deps,
            env,
            token_id,
            include_expired.unwrap_or(false),
        )?),
        QueryMsg::NftInfo { token_id } => to_binary(&query::nft_info(deps, token_id)?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_binary(&query::all_nft_info(
            deps,
            env,
            token_id,
            include_expired.unwrap_or_default(),
        )?),
        QueryMsg::NumTokens {} => to_binary(&query::num_tokens(deps)?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_binary(&query::tokens(deps, owner, start_after, limit)?),
        QueryMsg::ContractInfo {} => to_binary(&query::contract_info(deps)?),

        _ => StdResult::Ok(Default::default()),
    }
}
