use cosmwasm_std::{Addr, Binary, Deps, StdResult, to_binary};
use crate::msg::MoneyContractResponse;
use crate::state::CONFIG;

pub fn query_money_contract(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&MoneyContractResponse {
        address: config.money_cw20_contract.addr.unwrap()
    })
}
