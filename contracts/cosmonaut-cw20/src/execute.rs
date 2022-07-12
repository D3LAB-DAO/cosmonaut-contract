use crate::contract::{TokenExtension, TOKEN_EXTENSION};
use cosmwasm_std::{DepsMut, Response, Uint128};
use cw20_base::ContractError;

pub fn set_token_extension(deps: DepsMut, unit_weight: Uint128) -> Result<Response, ContractError> {
    TOKEN_EXTENSION.save(deps.storage, &TokenExtension { unit_weight })?;

    Ok(Response::new().add_attribute("action", "set_token_extension"))
}
