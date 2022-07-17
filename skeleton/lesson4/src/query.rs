use crate::msg::{ConfigResponse, FreightTokenBalanceResponse};
use crate::state::CONFIG;
use cosmonaut_cw20::msg::QueryMsg as Cw20QueryMsg;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::{to_binary, Binary, Deps, StdResult, Uint128};
use cw20::BalanceResponse;
use cw721::Cw721QueryMsg::NftInfo;
use cw721::{Cw721QueryMsg, NftInfoResponse, OwnerOfResponse};

pub fn config(deps: Deps) -> StdResult<Binary> {
    to_binary(&ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}

pub fn money_balance(deps: Deps, address: String) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let validated_addr = deps.api.addr_validate(&address)?;
    let res: BalanceResponse = deps.querier.query_wasm_smart(
        config.money_cw20_contract,
        &Cw20QueryMsg::Balance {
            address: validated_addr.to_string(),
        },
    )?;
    to_binary(&res)
}

pub fn owner_of_spaceship(deps: Deps, token_id: String) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let res: OwnerOfResponse = deps.querier.query_wasm_smart(
        config.spaceship_cw721_contract,
        &Cw721QueryMsg::OwnerOf {
            token_id,
            include_expired: None,
        },
    )?;
    to_binary(&res)
}

pub fn freight_token_balance(deps: Deps, symbol: String, address: String) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;

    let target_contract_addr = config
        .freight_contracts
        .into_iter()
        .find(|c| c.denom == symbol);

    match target_contract_addr {
        Some(f) => {
            let freight_contract_addr = f.address;
            let res: BalanceResponse = deps
                .querier
                .query_wasm_smart(freight_contract_addr, &Cw20QueryMsg::Balance { address })?;
            to_binary(&FreightTokenBalanceResponse {
                symbol,
                balance: res.balance,
            })
        }
        None => to_binary(&FreightTokenBalanceResponse {
            symbol,
            balance: Uint128::zero(),
        }),
    }
}

pub fn fuel_balance(deps: Deps, address: String) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let validated_addr = deps.api.addr_validate(&address)?;

    let res: BalanceResponse = deps.querier.query_wasm_smart(
        config.fuel_cw20_contract,
        &Cw20QueryMsg::Balance {
            address: validated_addr.to_string(),
        },
    )?;
    to_binary(&res)
}

pub fn spaceship_info(deps: Deps, token_id: String) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let res: NftInfoResponse<Extension> = deps
        .querier
        .query_wasm_smart(config.spaceship_cw721_contract, &NftInfo { token_id })?;
    to_binary(&res)
}
