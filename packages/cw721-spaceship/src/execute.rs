use cosmwasm_std::Addr;
use cw_multi_test::BasicApp;
use cosmonaut_cw721::state::Extension as cosmonautExtension;
use base::execute::execute_contract;

fn create_cw721_execute_msgs(
    admin: &str,
    recipient: &str,
    _stranger: &str,
) -> Vec<cosmonaut_cw721::msg::ExecuteMsg<cosmonautExtension>> {
    use cosmonaut_cw721::msg::ExecuteMsg;
    use cosmonaut_cw721::state::{Extension, Metadata};
    use cw721_base::MintMsg;

    let mint_msg = ExecuteMsg::<Extension>::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: admin.to_string(),
        token_uri: None,
        extension: Option::from(Metadata {
            unit_denom: "mars".to_string(),
            price: 500,
            name: Option::from("cosmonaut spaceship".to_string()),
            freight: vec![],
            health: 10,
        }),
    });

    let transfer_nft_msg = ExecuteMsg::<Extension>::TransferNft {
        recipient: recipient.to_string(),
        token_id: "1".to_string(),
    };

    vec![
        mint_msg,
        transfer_nft_msg,
    ]
}

pub fn execute_cw721_all_msg(
    mut app: BasicApp,
    contract_addr: Addr,
    admin: &str,
    recipient: &str,
    stranger: &str,
) -> BasicApp {
    use cosmonaut_cw721::msg::ExecuteMsg;

    let cw721_execute_msgs = create_cw721_execute_msgs(admin, recipient, stranger);
    for msg in cw721_execute_msgs {
        let execute_res = execute_contract::<ExecuteMsg<cosmonautExtension>>(app, &contract_addr, msg, &[], admin);
        for attr in execute_res.app_response.events {
            println!("{:?}", attr.attributes);
        };
        println!();
        app = execute_res.app
    }
    app
}
