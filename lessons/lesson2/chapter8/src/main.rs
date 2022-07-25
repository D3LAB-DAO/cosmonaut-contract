use base::consts::*;
use base::init::init_app;
use base::result::Result;
use cosmwasm_std::Uint128;
use cw20_tokens::execute::execute_cw20_all_msg;
use cw20_tokens::init::mock_cw20_contract;
use cw20_tokens::instantiate::instantiate_cw20_contract;
use cw20_tokens::query::query_all_cw20_msgs;

pub fn main() {
    let mut app = init_app(ADDR1);

    let money_cw20_code_id = app.store_code(mock_cw20_contract());
    let cw20_contract_addr = instantiate_cw20_contract(
        &mut app,
        money_cw20_code_id,
        ADDR1,
        ADDR1,
        "mars",
        "umars",
        vec![],
        Uint128::new(3),
        "cw20 money",
    );

    execute_cw20_all_msg(&mut app, &cw20_contract_addr.as_ref(), ADDR1, ADDR2)
        .check_answer(
            "2",
            "8",
            "/workspace/cargo-projects/cosm-base/answers/lesson2/chapter8/execute_result.json",
        ).print_serialized();
    // .write_answer_to_file(
    //     "2",
    //     "8",
    //     "../../../answers/lesson2/chapter8/execute_result.json");

    query_all_cw20_msgs(&app, &cw20_contract_addr, ADDR1, ADDR2)
        .check_answer(
            "2",
            "8",
            "/workspace/cargo-projects/cosm-base/answers/lesson2/chapter8/query_result.json",
        ).print_serialized();
    // .write_answer_to_file(
    //     "2",
    //     "8",
    //     "../../../answers/lesson2/chapter8/execute_result.json", );
}
