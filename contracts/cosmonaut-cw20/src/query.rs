use crate::msg::{AllowanceResponse, BalanceResponse, MintInfoResponse, TokenInfoResponse};
use crate::state::{ALLOWANCES, BALANCES, TOKEN_INFO};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult};

pub fn token_info(deps: Deps) -> StdResult<TokenInfoResponse> {
    let token_info = TOKEN_INFO.load(deps.storage)?;
    let res = TokenInfoResponse {
        name: token_info.name,
        symbol: token_info.symbol,
        decimals: token_info.decimals,
        total_supply: token_info.total_supply,
        unit_weight: token_info.unit_weight,
    };
    Ok(res)
}
