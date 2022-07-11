use crate::msg::{ConfigResponse, MoneyContractResponse};
use crate::state::CONFIG;
use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

pub fn query_money_contract(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&MoneyContractResponse {
        address: config.money_cw20_contract,
    })
}

pub fn query_config(deps: Deps) -> StdResult<Binary> {
    to_binary(&ConfigResponse {
        config: CONFIG.load(deps.storage)?,
    })
}
