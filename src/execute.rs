use crate::ContractError;
use cosmwasm_std::{from_binary, CosmosMsg, DepsMut, MessageInfo, Response};
use cw20::Cw20ReceiveMsg;
use cw721::Cw721ExecuteMsg;

pub fn execute_buy_spaceship(
    deps: DepsMut,
    info: MessageInfo,
    cw20_receive_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let sender_addr = deps.api.addr_validate(&cw20_receive_msg.sender)?;
    let msg: cw721::Cw721ExecuteMsg = from_binary(&cw20_receive_msg.msg)?;
    match msg {
        Cw721ExecuteMsg::TransferNft {
            recipient,
            token_id,
        } => Ok(Response::new()),
        _ => Err(ContractError::Unauthorized {}),
    }
}

pub fn execute_buy_supplies(
    deps: DepsMut,
    info: MessageInfo,
    contract: String,
    amount: u128,
) -> Result<Response, ContractError> {
    unimplemented!()
}
