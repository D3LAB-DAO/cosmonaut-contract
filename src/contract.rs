use crate::error::ContractError;
use crate::execute::{execute_buy_spaceship, execute_buy_supplies};
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response,
    StdResult, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20::{Cw20Coin, Cw20ReceiveMsg, MinterResponse};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw20_base::state::MinterData;
use cw721::Cw721ReceiveMsg;
use cw721_base::msg::InstantiateMsg as Cw721InstantiateMsg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmonaut-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        money_cw20_contract: msg.clone().money_cw20_contract,
        spaceship_cw721_contract: msg.clone().spaceship_cw721_contract,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    let instantiate_cw20_contract: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Option::from(info.sender.to_string()),
        code_id: msg.money_cw20_contract.code_id,
        msg: to_binary(&Cw20InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            decimals: 6,
            initial_balances: vec![Cw20Coin {
                address: info.sender.to_string(),
                amount: Uint128::new(1000000),
            }],
            mint: Option::from(MinterResponse {
                minter: info.sender.to_string(),
                cap: None,
            }),
            marketing: None,
        })?,
        funds: vec![],
        label: "mars token for money".to_string(),
    });

    let instantiate_cw721_contract: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Option::from(info.sender.to_string()),
        code_id: msg.spaceship_cw721_contract.code_id,
        msg: to_binary(&Cw721InstantiateMsg {
            name: "spaceship".to_string(),
            symbol: "SPACE".to_string(),
            minter: info.sender.to_string(),
        })?,
        funds: vec![],
        label: "spaceship nft".to_string(),
    });

    Ok(
        Response::new()
            .add_messages([instantiate_cw20_contract, instantiate_cw721_contract])
            .add_attribute("action", "instantiate")
            .add_attribute("sender", info.sender)
            .add_attribute("cw20_address", msg.money_cw20_contract.addr)
            .add_attribute("cw721_address", msg.spaceship_cw721_contract.addr)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Cw20ReceiveMsg(cw20_receive_msg) => {
            execute_buy_spaceship(deps, info, cw20_receive_msg)
        }
        _ => Ok(Response::new()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // match msg {
    //
    // }
    unimplemented!()
}
