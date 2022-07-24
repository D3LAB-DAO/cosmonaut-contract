use base::instantiate::instantiate_contract;
use cosmwasm_std::Addr;
use cw_multi_test::BasicApp;

pub fn instantiate_spaceship_nft_contract(
    app: &mut BasicApp,
    code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> Addr {
    use cw721_base::InstantiateMsg;
    let cw721_init_msg = InstantiateMsg {
        name: "spaceship".to_string(),
        symbol: "space".to_string(),
        minter: admin.to_string(),
    };
    instantiate_contract::<InstantiateMsg>(app, cw721_init_msg, code_id, sender, admin, label)
}
