use crate::state::{Extension, CONFIG};
use crate::ContractError;
use cosmwasm_std::{from_binary, to_binary, CosmosMsg, DepsMut, MessageInfo, Response, WasmMsg};
use cw20::Cw20ReceiveMsg;
use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, MintMsg};

pub fn execute_buy_spaceship(
    deps: DepsMut,
    info: MessageInfo,
    cw20_receive_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let cw20_contract_addr = info.sender.to_string();
    let message_from_cw20: Cw721ExecuteMsg<Extension> = from_binary(&cw20_receive_msg.msg)?;
    let mut messages: Vec<CosmosMsg> = vec![];

    let config = CONFIG.load(deps.storage)?;
    if cw20_contract_addr != config.spaceship_cw721_contract.addr {
        return Err(ContractError::InvalidContract {});
    }

    if let Cw721ExecuteMsg::Mint(mint_msg) = message_from_cw20 {
        let mint_new_nft_msg = MintMsg {
            token_id: mint_msg.token_id,
            owner: mint_msg.owner,
            token_uri: mint_msg.token_uri,
            extension: mint_msg.extension,
        };

        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: config.spaceship_cw721_contract.addr.to_string(),
            msg: to_binary(&mint_new_nft_msg)?,
            funds: vec![],
        }));
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "buy_spaceship"))
}

// pub fn execute_buy_supplies(
//     _deps: DepsMut,
//     _info: MessageInfo,
//     _contract: String,
//     _amount: u128,
// ) -> Result<Response, ContractError> {
//     unimplemented!()
// }
