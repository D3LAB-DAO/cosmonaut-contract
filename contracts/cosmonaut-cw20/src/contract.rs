use std::convert::TryInto;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw2::set_contract_version;
use cw20::{Cw20QueryMsg, MinterResponse};
use cw20_base::contract::{execute as cw20_execute, query as cw20_query, instantiate as cw20_instantiate};
use cw20_base::msg::{ExecuteMsg, InstantiateMsg};
use cw20_base::state::BALANCES;
use cw20_base::ContractError;
use cw_storage_plus::Item;
use crate::msg::QueryMsg;
use crate::query;
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

const CONTRACT_NAME: &str = "crates.io:mars-tokens";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenExtension {
    pub unit_weight: Uint128,
}

pub const TOKEN_EXTENSION: Item<TokenExtension> = Item::new("token_extension");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw20_instantiate(deps, _env, _info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    cw20_execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenExtension {} => to_binary(&query::token_extension(deps)?),
        _ => cw20_query(deps, env, msg.try_into()?)
    }
}
