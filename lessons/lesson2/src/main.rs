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

    println!(
        "{:?}",
        execute_cw20_all_msg(&mut app, &cw20_contract_addr.as_ref(), ADDR1, ADDR2).check_answer(
            which_lesson,
            &format!(
                "./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_execute_result.json",
                which_lesson, which_lesson
            ),
        )
    );
    // .write_answer_to_file(&format!(
    //     "./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_execute_result.json",
    //     which_lesson, which_lesson
    // ));

    println!(
        "{:?}",
        query_all_cw20_msgs(&app, &cw20_contract_addr, ADDR1, ADDR2).check_answer(
            which_lesson,
            &format!(
                "./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_query_result.json",
                which_lesson, which_lesson
            ),
        )
    );
    // .write_answer_to_file(&format!("./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_query_result.json", which_lesson, which_lesson));
}
