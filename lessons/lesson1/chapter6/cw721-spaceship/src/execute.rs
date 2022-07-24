use base::execute::execute_contract;
use base::result::ExecuteAllResult;
use cosmonaut_cw721::msg::ExecuteMsg;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::{Addr, Attribute, Uint128};
use cw721_base::MintMsg;
use cw_multi_test::BasicApp;

fn create_cw721_execute_msgs(
    admin: &str,
    recipient: &str,
    stranger: &str,
) -> Vec<cosmonaut_cw721::msg::ExecuteMsg> {
    let mint_msg_1 = ExecuteMsg::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: admin.to_string(),
        token_uri: None,
        extension: Extension {
            unit_denom: "mars".to_string(),
            price: Uint128::new(500),
            name: Some("cosmonaut spaceship1".to_string()),
            freights: vec![],
            health: Uint128::new(10),
            fuel: Uint128::zero(),
        },
    });

    let mint_msg_2 = ExecuteMsg::Mint(MintMsg {
        token_id: "2".to_string(),
        owner: admin.to_string(),
        token_uri: None,
        extension: Extension {
            unit_denom: "mars".to_string(),
            price: Uint128::new(700),
            name: Some("cosmonaut spaceship2".to_string()),
            freights: vec![],
            health: Uint128::new(15),
            fuel: Uint128::zero(),
        },
    });

    let transfer_nft_msg = ExecuteMsg::TransferNft {
        recipient: recipient.to_string(),
        token_id: "2".to_string(),
    };

    let approve_nft_msg = ExecuteMsg::Approve {
        spender: stranger.to_string(),
        token_id: "1".to_string(),
        expires: None,
    };

    let load_freight_msg = ExecuteMsg::LoadFreight {
        token_id: "1".to_string(),
        denom: "oil".to_string(),
        amount: Uint128::new(10000),
        unit_weight: Uint128::new(1),
    };

    let unload_freight_msg = ExecuteMsg::UnloadFreight {
        token_id: "1".to_string(),
        denom: "oil".to_string(),
        amount: Uint128::new(5000),
    };

    let decrease_health_msg = ExecuteMsg::DecreaseHealth {
        token_id: "1".to_string(),
        value: Uint128::new(5),
    };

    let fuel_up_msg = ExecuteMsg::FuelUp {
        token_id: "1".to_string(),
        amount: Uint128::new(100),
    };

    let burn_fuel_msg = ExecuteMsg::BurnFuel {
        token_id: "1".to_string(),
        amount: Uint128::new(50),
    };

    vec![
        mint_msg_1,
        mint_msg_2,
        approve_nft_msg,
        load_freight_msg,
        unload_freight_msg,
        decrease_health_msg,
        transfer_nft_msg,
        fuel_up_msg,
        burn_fuel_msg,
    ]
}

pub fn execute_cw721_all_msg(
    app: &mut BasicApp,
    contract_addr: &str,
    admin: &str,
    recipient: &str,
    stranger: &str,
) -> ExecuteAllResult {
    let mut total_attributes: Vec<Vec<Attribute>> = vec![];
    let mut total_errors: Vec<String> = vec![];

    let cw721_execute_msgs = create_cw721_execute_msgs(admin, recipient, stranger);

    for msg in cw721_execute_msgs {
        let execute_res =
            execute_contract::<ExecuteMsg>(app, &Addr::unchecked(contract_addr), &msg, &[], admin);
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
