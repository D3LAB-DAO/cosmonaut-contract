use crate::state::{CONFIG, Extension};
use crate::ContractError;
use cw721::{Cw721Query, Cw721QueryMsg, NftInfoResponse};
use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, MessageInfo, Response, WasmMsg, WasmQuery, QuerierWrapper};

pub fn execute_buy_spaceship(
    deps: DepsMut,
    info: MessageInfo,
    nft_id: String,
    original_owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let nft_info: NftInfoResponse<Extension> = deps.querier.query_wasm_smart(
        config.spaceship_cw721_contract.addr.unwrap().to_string(),
        &to_binary(&Cw721QueryMsg::NftInfo {
            token_id: nft_id
        })?,
    )?;


    let mut messages: Vec<CosmosMsg> = vec![];

    Ok(Response::new()
        .add_messages(messages)
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
