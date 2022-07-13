use crate::contract::{TokenExtension, TOKEN_EXTENSION};
use cosmwasm_std::{Deps, StdResult};

pub fn token_extension(deps: Deps) -> StdResult<TokenExtension> {
    let token_extension = TOKEN_EXTENSION.load(deps.storage)?;

    Ok(TokenExtension {
        unit_weight: token_extension.unit_weight,
    })
}
