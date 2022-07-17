use base::consts::*;
use base::init::init_app;
use base::result::Result;
use cosmwasm_std::Uint128;
use cw20_tokens::execute::execute_cw20_all_msg;
use cw20_tokens::init::mock_cw20_contract;
use cw20_tokens::instantiate::instantiate_cw20_contract;
use cw20_tokens::query::query_all_cw20_msgs;
use std::env::args;

pub fn main() {
    if args().count() != 2 {
        println!("args are not sufficient");
        std::process::exit(0);
    }

    let which_lesson: &str = &args().nth(1).unwrap();

    let volume_path: &str = "/Users/ogsang-yun/Documents/IdeaProjects/cosmonaut-contract";


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
            which_lesson,
            &format!("{}/answers/lesson{}/lesson{}_execute_result.json", volume_path, which_lesson, which_lesson),
        ).print_serialized();
    // .write_answer_to_file(
    //     &format!("{}/answers/lesson{}/lesson{}_execute_result.json", volume_path, which_lesson, which_lesson)
    // );

    query_all_cw20_msgs(&app, &cw20_contract_addr, ADDR1, ADDR2)
        .check_answer(
            which_lesson,
            &format!("{}/answers/lesson{}/lesson{}_query_result.json", volume_path, which_lesson, which_lesson),
        ).print_serialized();

    // .write_answer_to_file(&format!("{}/answers/lesson{}/lesson{}_query_result.json", volume_path, which_lesson, which_lesson));
}
