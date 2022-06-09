use base::instantiate::instantiate_contract;
use cosmwasm_std::Addr;
use cw_multi_test::BasicApp;

pub fn instantiate_main_contract(
    app: &mut BasicApp,
    code_id: u64,
    money_cw20_code_id: u64,
    spaceship_cw721_code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> Addr {
    use cosmonaut_main::msg::{ContractInitInfo, InstantiateMsg};

    let main_init_msg = InstantiateMsg {
        money_cw20_contract: ContractInitInfo {
            addr: None,
            code_id: money_cw20_code_id,
        },
        spaceship_cw721_contract: ContractInitInfo {
            addr: None,
            code_id: spaceship_cw721_code_id,
        },
    };
    instantiate_contract::<InstantiateMsg>(app, main_init_msg, code_id, sender, admin, label)
}
