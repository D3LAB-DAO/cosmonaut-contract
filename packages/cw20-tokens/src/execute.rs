use base::execute::execute_contract;
use base::result::ExecuteAllResult;
use cosmonaut_cw20::msg::ExecuteMsg;
use cosmwasm_std::{Addr, Attribute, Uint128};
use cw_multi_test::BasicApp;

fn create_cw20_execute_msgs_before_allowance(admin: &str, recipient: &str) -> Vec<ExecuteMsg> {
    let mint_msg = ExecuteMsg::Mint {
        recipient: admin.to_string(),
        amount: Uint128::new(10000),
    };

    let transfer_msg = ExecuteMsg::Transfer {
        recipient: recipient.to_string(),
        amount: Uint128::new(1000),
    };

    let burn_msg = ExecuteMsg::Burn {
        amount: Uint128::new(10),
    };

    let increase_allowance_msg = ExecuteMsg::IncreaseAllowance {
        spender: recipient.to_string(),
        amount: Uint128::new(5000),
        expires: None,
    };

    let decrease_allowance_msg = ExecuteMsg::DecreaseAllowance {
        spender: recipient.to_string(),
        amount: Uint128::new(30),
        expires: None,
    };

    vec![
        mint_msg,
        transfer_msg,
        burn_msg,
        increase_allowance_msg,
        decrease_allowance_msg,
    ]
}

pub fn execute_cw20_all_msg(
    app: &mut BasicApp,
    contract_addr: &str,
    admin: &str,
    recipient: &str,
) -> ExecuteAllResult {
    let mut total_attributes: Vec<Vec<Attribute>> = vec![];
    let mut total_errors: Vec<String> = vec![];

    let cw20_execute_msgs_before_allowance =
        create_cw20_execute_msgs_before_allowance(admin, recipient);

    for msg in cw20_execute_msgs_before_allowance {
        let execute_res = execute_contract(app, &Addr::unchecked(contract_addr), &msg, &[], admin);
        match execute_res {
            Ok(res) => total_attributes.push(res),
            Err(err) => total_errors.push(err.root_cause().to_string()),
        }
    }

    let transfer_from_msg = ExecuteMsg::TransferFrom {
        owner: admin.to_string(),
        recipient: recipient.to_string(),
        amount: Uint128::new(300),
    };

    let burn_from_msg = ExecuteMsg::BurnFrom {
        owner: admin.to_string(),
        amount: Uint128::new(10),
    };

    let transfer_from_res = execute_contract(
        app,
        &Addr::unchecked(contract_addr),
        &transfer_from_msg,
        &[],
        recipient,
    );
    match transfer_from_res {
        Ok(res) => total_attributes.push(res),
        Err(err) => total_errors.push(err.root_cause().to_string()),
    }

    let burn_from_res = execute_contract(
        app,
        &Addr::unchecked(contract_addr),
        &burn_from_msg,
        &[],
        recipient,
    );
    match burn_from_res {
        Ok(res) => total_attributes.push(res),
        Err(err) => total_errors.push(err.root_cause().to_string()),
    }

    ExecuteAllResult {
        attributes: total_attributes,
        errors: total_errors,
    }
}
