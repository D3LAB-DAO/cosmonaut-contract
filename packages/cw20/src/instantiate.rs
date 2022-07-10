use base::instantiate::instantiate_contract;
use cosmwasm_std::{Addr, Uint128};
use cw_multi_test::BasicApp;

// function to generate cw20 contracts
pub fn instantiate_cw20_contract(
    app: &mut BasicApp,
    code_id: u64,
    sender: &str,
    admin: &str,
    name: &str,
    symbol: &str,
    unit_weight: Option<Uint128>,
    label: &str,
) -> Addr {
    use cosmonaut_cw20::msg::{InstantiateMsg, MinterResponse};

    let cw20_init_msg = InstantiateMsg {
        name: name.to_string(),
        symbol: symbol.to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: admin.to_string(),
            cap: None,
        }),
        marketing: None,
        total_supply: Uint128::new(u128::MAX),
        unit_weight,
    };
    instantiate_contract::<InstantiateMsg>(app, cw20_init_msg, code_id, sender, admin, label)
}
