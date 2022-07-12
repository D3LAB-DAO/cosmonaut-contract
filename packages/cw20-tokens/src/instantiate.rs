use base::instantiate::instantiate_contract;
use cosmwasm_std::{Addr, Uint128};
use cw20::{Cw20Coin, MinterResponse};
use cw20_base::msg::InstantiateMsg;
use cw_multi_test::BasicApp;

// function to generate cw20-tokens contracts
pub fn instantiate_cw20_contract(
    app: &mut BasicApp,
    code_id: u64,
    sender: &str,
    admin: &str,
    name: &str,
    symbol: &str,
    initial_balances: Vec<Cw20Coin>,
    unit_weight: Uint128,
    label: &str,
) -> Addr {
    let mut total_supply = Uint128::zero();
    for i in &initial_balances {
        total_supply = total_supply.checked_add(i.amount).unwrap();
    }
    let cw20_init_msg = InstantiateMsg {
        name: name.to_string(),
        symbol: symbol.to_string(),
        decimals: 6,
        initial_balances,
        mint: Some(MinterResponse {
            minter: admin.to_string(),
            cap: None,
        }),
        marketing: None,
    };
    instantiate_contract::<InstantiateMsg>(app, cw20_init_msg, code_id, sender, admin, label)
}
