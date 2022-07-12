use cosmwasm_std::{Deps, StdResult};
use cw20::TokenInfoResponse;
use crate::contract::{TOKEN_EXTENSION, TokenExtension};

pub fn token_extension(
    deps: Deps
) -> StdResult<TokenExtension> {
    let token_extension = TOKEN_EXTENSION.load(deps.storage)?;

    Ok(TokenExtension {
        unit_weight: token_extension.unit_weight
    })
}
