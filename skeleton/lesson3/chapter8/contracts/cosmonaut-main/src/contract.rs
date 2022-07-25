use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::{execute, query};

use cosmonaut_cw20::contract::TokenExtension;
use cosmonaut_cw20::msg::InstantiateMsg as Cw20InstantiateMsg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, Uint128, WasmMsg,
};
use cw2::set_contract_version;
use cw20::MinterResponse;
use cw721_base::msg::InstantiateMsg as Cw721InstantiateMsg;
use cw_utils::parse_reply_instantiate_data;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cosmonaut-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const CW20_MONEY_CONTRACT_REPLY_ID: u64 = 1;
const CW20_FUEL_CONTRACT_REPLY_ID: u64 = 2;
const CW721_SPACESHIP_CONTRACT_REPLY_ID: u64 = 3;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        money_cw20_contract: Addr::unchecked(""),
        spaceship_cw721_contract: Addr::unchecked(""),
        fuel_cw20_contract: Addr::unchecked(""),
        freight_contracts: vec![],
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    // Submessage to instantiate cw20 money contract
    // if instantiate successes, trigger reply function as callback
    // https://docs.cosmwasm.com/docs/1.0/smart-contracts/message/submessage/#creating-a-submessage
    let instantiate_cw20_money_contract: SubMsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.money_cw20_id,
            msg: to_binary(&Cw20InstantiateMsg {
                name: "MARS".to_string(),
                symbol: "mars".to_string(),
                decimals: 6,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: env.contract.address.to_string(),
                    cap: None,
                }),
                marketing: None,
                token_extension: Some(TokenExtension {
                    unit_weight: Uint128::new(0),
                }),
            })?,
            funds: vec![],
            label: "mars token for money".to_string(),
        },
        CW20_MONEY_CONTRACT_REPLY_ID,
    );

    let instantiate_cw20_fuel_contract: SubMsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.fuel_cw20_id,
            msg: to_binary(&Cw20InstantiateMsg {
                name: "fuel".to_string(),
                symbol: "ufuel".to_string(),
                decimals: 6,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: env.contract.address.to_string(),
                    cap: None,
                }),
                marketing: None,
                token_extension: Some(TokenExtension {
                    unit_weight: Uint128::new(1),
                }),
            })?,
            funds: vec![],
            label: "fuel token for game".to_string(),
        },
        CW20_FUEL_CONTRACT_REPLY_ID,
    );

    let instantiate_cw721_spaceship_contract: SubMsg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.spaceship_cw721_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: "spaceship".to_string(),
                symbol: "SPACE".to_string(),
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            label: "spaceship nft".to_string(),
        },
        CW721_SPACESHIP_CONTRACT_REPLY_ID,
    );

    Ok(Response::new()
        // TODO: q1) Add submessages created above using add_submessages
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    match msg.id {
        CW20_MONEY_CONTRACT_REPLY_ID => handle_cw20_money_instantiate_reply(deps, msg),
        CW20_FUEL_CONTRACT_REPLY_ID => handle_cw20_fuel_instantiate_reply(deps, msg),
        CW721_SPACESHIP_CONTRACT_REPLY_ID => handle_cw721_spaceship_instantiate_reply(deps, msg),
        _ => Err(StdError::not_found("not found")),
    }
}

fn handle_cw20_money_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg).unwrap();
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.money_cw20_contract = Addr::unchecked(res.contract_address);
        Ok(config)
    })?;
    Ok(Response::new())
}

fn handle_cw20_fuel_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let res = parse_reply_instantiate_data(msg).unwrap();
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.fuel_cw20_contract = Addr::unchecked(res.contract_address);
        Ok(config)
    })?;
    Ok(Response::new())
}

fn handle_cw721_spaceship_instantiate_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // TODO: q2) save spaceship_cw721_contract address to CONFIG

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddFreightContract { address } => execute::add_freight_contract(deps, address),
        ExecuteMsg::BuyNft { nft_id } => execute::buy_spaceship(deps, env, info, nft_id),

        ExecuteMsg::Mint(mint_msg) => execute::mint_to_cw721_contract(deps, info, mint_msg),

        ExecuteMsg::SetMinter { minter } => execute::set_minter_to_cw721_contract(deps, minter),
        ExecuteMsg::LoadFreight {
            address,
            token_id,
            amount,
        } => execute::load_freight_to_nft(deps, info, address, token_id, amount),
        ExecuteMsg::UnLoadFreight {
            address,
            token_id,
            amount,
        } => execute::unload_freight_from_nft(deps, info, address, token_id, amount),
        ExecuteMsg::BuyMoneyToken { amount } => execute::buy_money_token(deps, info, amount),
        ExecuteMsg::BuyFreightToken { address, amount } => {
            execute::buy_freight_token(deps, info, address, amount)
        }
        ExecuteMsg::BuyFuelToken { amount } => execute::buy_fuel_token(deps, info, amount),
        ExecuteMsg::FuelUp { token_id, amount } => execute::fuel_up(deps, info, token_id, amount),
        ExecuteMsg::BurnFuel { token_id, amount } => execute::burn_fuel(deps, token_id, amount),
        ExecuteMsg::PlayGame { token_id, epoch } => execute::play_game(deps, env, token_id, epoch),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::MoneyBalance { address } => query::money_balance(deps, address),
        QueryMsg::OwnerOfSpaceShip { token_id } => query::owner_of_spaceship(deps, token_id),
        QueryMsg::FreightTokenBalance { symbol, address } => {
            query::freight_token_balance(deps, symbol, address)
        }
        QueryMsg::FuelBalance { address } => query::fuel_balance(deps, address),
        QueryMsg::SpaceShipInfo { token_id } => query::spaceship_info(deps, token_id),
        QueryMsg::Config {} => query::config(deps),
    }
}
