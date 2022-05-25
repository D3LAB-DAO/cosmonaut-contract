use crate::state::{CONFIG, Extension};
use crate::ContractError;
use cw721::{Cw721QueryMsg, NftInfoResponse};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, MessageInfo, Response, Addr};

pub fn execute_buy_spaceship(
    deps: DepsMut,
    _info: MessageInfo,
    nft_id: String,
    _original_owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let nft_info: NftInfoResponse<Extension> = deps.querier.query_wasm_smart(
        Addr::unchecked(config.spaceship_cw721_contract.addr.unwrap()),
        &to_binary(&Cw721QueryMsg::NftInfo {
            token_id: nft_id
        })?,
    )?;


    Ok(Response::new()
        .add_attribute("action", "buy_spaceship")
        .add_attribute("query", nft_info.extension.unwrap().price.to_string())
    )
}

// pub fn execute_buy_supplies(
//     _deps: DepsMut,
//     _info: MessageInfo,
//     _contract: String,
//     _amount: u128,
// ) -> Result<Response, ContractError> {
//     unimplemented!()
// }
