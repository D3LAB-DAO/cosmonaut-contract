use base::execute::execute_contract;
use base::result::ExecuteAllResult;
use cosmonaut_cw721::state::{Extension, Metadata};
use cosmonaut_main::msg::{ConfigResponse, ExecuteMsg, QueryMsg};
use cosmwasm_std::{Addr, Attribute, coin, StdError, Uint128};
use cw721_base::MintMsg;
use cw_multi_test::BasicApp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmonaut_cw721::ContractError;
use cosmonaut_main::contract::execute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FreightParams {
    pub contract_addr: String,
    pub amount: Uint128,
}

fn create_main_contract_execute_msgs_before_approve(
    main_contract_addr: &str,
    recipient: &str,
) -> Vec<cosmonaut_main::msg::ExecuteMsg> {
    let buy_money_token_msg = ExecuteMsg::BuyMoneyToken { amount: Uint128::new(10000) };
    let mint_msg = ExecuteMsg::Mint(MintMsg {
        token_id: 1.to_string(),
        owner: recipient.to_string(),
        token_uri: None,
        extension: Metadata {
            unit_denom: "mars".to_string(),
            price: 500,
            name: Some("cosmonaut spaceship".to_string()),
            freight: vec![],
            health: 10,
        },
    });
    vec![
        buy_money_token_msg,
        mint_msg,
    ]
}

fn create_main_contract_execute_msgs(
    main_contract_addr: &str,
    freights: Vec<FreightParams>,
) -> Vec<cosmonaut_main::msg::ExecuteMsg> {
    let buy_nft_msg = ExecuteMsg::BuyNft {
        original_owner: main_contract_addr.to_string(),
        nft_id: "1".to_string(),
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

    // let msg_except_freight_vec = vec![
    //     buy_nft_msg,
    //     play_game_msg
    // ];
    //
    // [freight_msgs, msg_except_freight_vec].concat()

    [vec![buy_nft_msg], freight_msgs].concat()
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

    let main_contract_config: ConfigResponse = app.wrap().query_wasm_smart(main_contract_addr, &QueryMsg::Config {}).unwrap();

    let main_execute_msgs_before_approve = create_main_contract_execute_msgs_before_approve(main_contract_addr, recipient);
    for msg in main_execute_msgs_before_approve {
        let execute_res = execute_contract(app, &Addr::unchecked(main_contract_addr), &msg, &[coin(100000, "uatom")], admin);
        match execute_res {
            Ok(res) => total_attributes.push(res),
            Err(err) => total_errors.push(err.root_cause().to_string())
        }
    }

    let approve_nft_msg = cosmonaut_cw721::msg::ExecuteMsg::Approve {
        spender: main_contract_addr.to_string(),
        token_id: "1".to_string(),
        expires: None,
    };

    let increase_allowance_msg = cosmonaut_cw20::msg::ExecuteMsg::IncreaseAllowance {
        spender: main_contract_addr.to_string(),
        amount: Uint128::new(10000),
        expires: None,
    };

    execute_contract(app, &Addr::unchecked(main_contract_config.config.spaceship_cw721_contract), &approve_nft_msg, &[], recipient);
    execute_contract(app, &Addr::unchecked(main_contract_config.config.money_cw20_contract), &increase_allowance_msg, &[], admin);

    for freight in freights.clone() {
        let addr = freight.contract_addr;
        execute_contract(app, &Addr::unchecked(addr), &increase_allowance_msg, &[], admin);
    }

    let main_execute_msgs = create_main_contract_execute_msgs(admin, freights);
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
