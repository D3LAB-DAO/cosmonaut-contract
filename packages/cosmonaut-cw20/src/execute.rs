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

    if mint.minter != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let minter = deps.api.addr_validate(&new_minter)?;

    let minter_data = MinterData {
        minter,
        cap: mint.cap,
    };

    config.mint = Some(minter_data);

    TOKEN_INFO.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_minter")
        .add_attribute("new_minter", new_minter))
}
