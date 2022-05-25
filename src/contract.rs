use crate::error::ContractError;
use crate::execute::execute_buy_spaceship;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20::{Cw20Coin, MinterResponse};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw721_base::msg::InstantiateMsg as Cw721InstantiateMsg;
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmonaut-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CW20_CONTRACT_REPLY_ID: u64 = 1;
const CW721_CONTRACT_REPLY_ID: u64 = 2;

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

    let instantiate_cw20_contract: SubMsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
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
        },
        CW20_CONTRACT_REPLY_ID,
    );

    let instantiate_cw721_contract: SubMsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Option::from(info.sender.to_string()),
            code_id: msg.spaceship_cw721_contract.code_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: "spaceship".to_string(),
                symbol: "SPACE".to_string(),
                minter: info.sender.to_string(),
            })?,
            funds: vec![],
            label: "spaceship nft".to_string(),
        },
        CW721_CONTRACT_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessages([instantiate_cw20_contract, instantiate_cw721_contract])
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        CW20_CONTRACT_REPLY_ID => handle_instantiate_reply(deps, msg),
        CW721_CONTRACT_REPLY_ID => handle_instantiate_reply(deps, msg),
        _ => Err(StdError::not_found("not found")),
    }
}

fn handle_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg.clone()).unwrap();
    match msg.id {
        CW20_CONTRACT_REPLY_ID => {
            CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
                config.money_cw20_contract.addr =
                    Option::from(Addr::unchecked(res.contract_address));
                Ok(config)
            })?;
        }
        CW721_CONTRACT_REPLY_ID => {
            CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
                config.spaceship_cw721_contract.addr =
                    Option::from(Addr::unchecked(res.contract_address));
                Ok(config)
            })?;
        }
        _ => {}
    }
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BuyNft { nft_id, original_owner } => {
            execute_buy_spaceship(deps, info, nft_id, original_owner)
        }
        _ => Ok(Response::new()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // match msg {
    //
    // }
    unimplemented!()
}
