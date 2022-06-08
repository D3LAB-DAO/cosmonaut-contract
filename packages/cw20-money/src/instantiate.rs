use base::instantiate::instantiate_contract;
use base::result::InstantiateResult;
use cw_multi_test::BasicApp;

pub fn instantiate_cw20_money_contract(
    app: BasicApp,
    code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> InstantiateResult {
    use cosmonaut_cw20::msg::{InstantiateMsg, MinterResponse};

    let cw20_init_msg = InstantiateMsg {
        name: "mars".to_string(),
        symbol: "umars".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Option::from(MinterResponse {
            minter: admin.to_string(),
            cap: None,
        }),
        marketing: None,
        total_supply: None,
    };
    instantiate_contract::<InstantiateMsg>(app, cw20_init_msg, code_id, sender, admin, label)
}
