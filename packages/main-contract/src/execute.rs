use base::execute::execute_contract;
use base::result::ExecuteAllResult;
use cosmonaut_cw721::state::{Extension, Metadata};
use cosmonaut_main::msg::ExecuteMsg;
use cosmwasm_std::{Addr, Attribute, Uint128};
use cw721_base::MintMsg;
use cw_multi_test::BasicApp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreightParams {
    pub contract_addr: String,
    pub amount: Uint128,
}

fn create_main_contract_execute_msgs(
    admin: &str,
    recipient: &str,
    freights: Vec<FreightParams>,
) -> Vec<cosmonaut_main::msg::ExecuteMsg> {
    let buy_money_token_msg = ExecuteMsg::BuyMoneyToken { amount: Uint128::new(1000) };

    let buy_nft_msg = ExecuteMsg::BuyNft {
        original_owner: admin.to_string(),
        nft_id: 1.to_string(),
    };

    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: 1.to_string(),
        owner: admin.to_string(),
        token_uri: None,
        extension: Metadata {
            unit_denom: "mars".to_string(),
            price: 500,
            name: Some("cosmonaut spaceship".to_string()),
            freight: vec![],
            health: 10,
        },
    });

    let set_minter_msg = ExecuteMsg::SetMinter {
        minter: recipient.to_string(),
    };

    let mut freight_msgs = vec![];

    for i in freights {
        let add_freight_contract_msg = ExecuteMsg::AddFreightContract {
            address: i.clone().contract_addr,
        };

        let buy_freight_token_msg = ExecuteMsg::BuyFreightToken {
            address: i.clone().contract_addr,
            amount: i.amount,
        };

        let load_freight_msg = ExecuteMsg::LoadFreight {
            address: i.clone().contract_addr,
            token_id: "1".to_string(),
            amount: i.amount.multiply_ratio(1u128, 2u128),
        };

        let unload_freight_msg = ExecuteMsg::UnLoadFreight {
            address: i.clone().contract_addr,
            token_id: "1".to_string(),
            amount: i.amount.multiply_ratio(1u128, 4u128),
        };

        freight_msgs.push(add_freight_contract_msg);
        freight_msgs.push(buy_freight_token_msg);
        freight_msgs.push(load_freight_msg);
        freight_msgs.push(unload_freight_msg);
    }

    let play_game_msg = ExecuteMsg::PlayGame {
        token_id: 1.to_string(),
        epoch: 5,
    };

    let msg_except_freight_vec = vec![
        buy_money_token_msg,
        buy_nft_msg,
        mint_msg,
        set_minter_msg,
        play_game_msg,
    ];

    [freight_msgs, msg_except_freight_vec].concat()
}

pub fn execute_main_all_msg(
    app: &mut BasicApp,
    main_contract_addr: &str,
    freights: Vec<FreightParams>,
    admin: &str,
    recipient: &str,
) -> ExecuteAllResult {
    let mut total_attributes: Vec<Vec<Attribute>> = vec![];
    let mut total_errors: Vec<String> = vec![];

    let main_execute_msgs = create_main_contract_execute_msgs(admin, recipient, freights);
    for msg in main_execute_msgs {
        let execute_res =
            execute_contract(app, &Addr::unchecked(main_contract_addr), &msg, &[], admin);
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
