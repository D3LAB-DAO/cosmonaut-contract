use crate::msg::ExecuteMsg;
use crate::state::{Extension, Metadata, CONFIG};
use crate::ContractError;
use cosmwasm_std::{to_binary, Addr, CosmosMsg, DepsMut, MessageInfo, Response, WasmMsg};
use cw721::{Cw721QueryMsg, NftInfoResponse};
use cw721_base::ExecuteMsg::Mint;
use cw721_base::MintMsg;

pub fn execute_mint_to_cw721_contract(
    deps: DepsMut,
    _info: MessageInfo,
    mint_msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let mint_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
        msg: to_binary(&Mint(mint_msg))?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_message(mint_msg_wrap))
}

pub fn execute_buy_spaceship(
    deps: DepsMut,
    nft_id: String,
    _original_owner: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let nft_info: NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
        Addr::unchecked(config.spaceship_cw721_contract.addr.as_ref().unwrap()),
        &Cw721QueryMsg::NftInfo { token_id: nft_id },
    )?;

    Ok(Response::new()
        .add_attribute("action", "buy_spaceship")
        .add_attribute("price", nft_info.extension.price.to_string()))
}

pub fn execute_set_minter_to_cw721_contract(
    deps: DepsMut,
    minter: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let set_minter_msg: ExecuteMsg<Extension> = ExecuteMsg::SetMinter { minter };

    let set_minter_msg_wrapper = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.spaceship_cw721_contract.addr.unwrap().to_string(),
        msg: to_binary(&set_minter_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_message(set_minter_msg_wrapper))
}
