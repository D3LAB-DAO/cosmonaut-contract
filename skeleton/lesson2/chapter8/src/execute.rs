use crate::contract::{TokenExtension, TOKEN_EXTENSION};
use cosmwasm_std::{DepsMut, MessageInfo, Response, Uint128};
use cw20_base::state::{MinterData, TOKEN_INFO};
use cw20_base::ContractError;

pub fn set_token_extension(deps: DepsMut, unit_weight: Uint128) -> Result<Response, ContractError> {
    TOKEN_EXTENSION.save(deps.storage, &TokenExtension { unit_weight })?;

    Ok(Response::new().add_attribute("action", "set_token_extension"))
}

pub fn execute_update_minter(
    deps: DepsMut,
    info: MessageInfo,
    new_minter: String,
) -> Result<Response, ContractError> {
    let mut config = TOKEN_INFO.load(deps.storage)?;
    let mint = config.mint.as_ref().ok_or(ContractError::Unauthorized {})?;

    // TODO: q3) check if mint.minter is same as info.sender, else return ContractError::Unauthorized.

    // validate new_minter address
    let minter = deps.api.addr_validate(&new_minter)?;

    let minter_data = MinterData {
        minter,
        cap: mint.cap,
    };

    // TODO q4) save config.mint with minter_data.
    // after that, save TOKEN_INFO with reference of config.

    Ok(Response::new()
        .add_attribute("action", "update_minter")
        .add_attribute("new_minter", new_minter))
}
