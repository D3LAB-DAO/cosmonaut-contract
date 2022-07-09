use base::execute::execute_contract;
use base::result::ExecuteAllResult;
use cosmonaut_cw721::state::Extension as cosmonautExtension;
use cosmwasm_std::{Addr, Attribute};
use cw_multi_test::BasicApp;

fn create_cw721_execute_msgs(
    admin: &str,
    recipient: &str,
    stranger: &str,
) -> Vec<cosmonaut_cw721::msg::ExecuteMsg<cosmonautExtension>> {
    use cosmonaut_cw721::msg::ExecuteMsg;
    use cosmonaut_cw721::state::{Extension, Metadata};
    use cw721_base::MintMsg;

    let mint_msg = ExecuteMsg::<Extension>::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: admin.to_string(),
        token_uri: None,
        extension: Some(Metadata {
            unit_denom: "mars".to_string(),
            price: 500,
            name: Some("cosmonaut spaceship".to_string()),
            freight: vec![],
            health: 10,
        }),
    });

    let transfer_nft_msg = ExecuteMsg::<Extension>::TransferNft {
        recipient: recipient.to_string(),
        token_id: "1".to_string(),
    };

    let approve_nft_msg = ExecuteMsg::<Extension>::Approve {
        spender: stranger.to_string(),
        token_id: "1".to_string(),
        expires: None,
    };

    let load_freight_msg = ExecuteMsg::<Extension>::LoadFreight {
        token_id: "1".to_string(),
        denom: "oil".to_string(),
        amount: 10000,
        unit_weight: 1,
    };

    let unload_freight_msg = ExecuteMsg::<Extension>::UnloadFreight {
        token_id: "1".to_string(),
        denom: "oil".to_string(),
        amount: 5000,
    };

    let decrease_health_msg = ExecuteMsg::<Extension>::DecreaseHealth {
        token_id: "1".to_string(),
        value: 5,
    };

    vec![
        mint_msg,
        approve_nft_msg,
        load_freight_msg,
        unload_freight_msg,
        decrease_health_msg,
        transfer_nft_msg,
    ]
}

pub fn execute_cw721_all_msg(
    app: &mut BasicApp,
    contract_addr: &str,
    admin: &str,
    recipient: &str,
    stranger: &str,
) -> ExecuteAllResult {
    use cosmonaut_cw721::msg::ExecuteMsg;
    let mut total_attributes: Vec<Vec<Attribute>> = vec![];
    let mut total_errors: Vec<String> = vec![];

    let cw721_execute_msgs = create_cw721_execute_msgs(admin, recipient, stranger);

    for msg in cw721_execute_msgs {
        let execute_res = execute_contract::<ExecuteMsg<cosmonautExtension>>(
            app,
            &Addr::unchecked(contract_addr),
            &msg,
            &[],
            admin,
        );
        match execute_res {
            Ok(res) => total_attributes.push(res),
            Err(err) => total_errors.push(err.root_cause().to_string()),
        }
    }

    ExecuteAllResult {
        attributes: total_attributes,
        errors: total_errors,
    }
}
