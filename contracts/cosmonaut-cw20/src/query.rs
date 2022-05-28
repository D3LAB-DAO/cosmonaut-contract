use crate::msg::{AllowanceResponse, BalanceResponse, MintInfoResponse, TokenInfoResponse};
use crate::state::{ALLOWANCES, BALANCES, TOKEN_INFO};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, StdResult};

pub fn query_balance(deps: Deps, address: String) -> StdResult<Binary> {
    let balance = BALANCES
        .may_load(deps.storage, &Addr::unchecked(address))?
        .unwrap_or_default();
    to_binary(&BalanceResponse { balance })
}

pub fn query_token_info(deps: Deps) -> StdResult<Binary> {
    let token_info = TOKEN_INFO.load(deps.storage)?;
    to_binary(&TokenInfoResponse {
        name: token_info.name,
        symbol: token_info.symbol,
        decimals: token_info.decimals,
        total_supply: token_info.total_supply.unwrap_or_default(),
    })
}

pub fn query_mint_info(deps: Deps) -> StdResult<Binary> {
    let mint_info = TOKEN_INFO.load(deps.storage)?;
    let minter = mint_info.mint.unwrap();
    to_binary(&MintInfoResponse {
        minter: minter.minter.to_string(),
        cap: minter.cap,
    })
}

pub fn query_allowance(deps: Deps, owner: String, spender: String) -> StdResult<Binary> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let spender_addr = deps.api.addr_validate(&spender)?;
    let allowance_info = ALLOWANCES
        .may_load(deps.storage, (&owner_addr, &spender_addr))?
        .unwrap_or_default();
    to_binary(&AllowanceResponse {
        allowance: allowance_info.allowance,
        expires: allowance_info.expires,
    })
}
