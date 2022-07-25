use base::consts::*;
use base::init::init_app;
use base::result::Result;
use cosmwasm_std::Uint128;
use cw20_tokens::init::mock_cw20_contract;
use cw20_tokens::instantiate::instantiate_cw20_contract;
use cw721_spaceship::init::mock_cw721_contract;
use main_contract::execute::{execute_main_all_msg, FreightParams};
use main_contract::init::mock_main_contract;
use main_contract::instantiate::instantiate_main_contract;
use main_contract::query::query_all_main_contract_msgs;


pub fn main() {
    let mut app = init_app(ADDR1);

    let money_cw20_code_id = app.store_code(mock_cw20_contract());
    let fuel_cw20_code_id = app.store_code(mock_cw20_contract());
    let bullet_cw20_code_id = app.store_code(mock_cw20_contract());
    let spaceship_cw721_code_id = app.store_code(mock_cw721_contract());
    let main_contract_id = app.store_code(mock_main_contract());

    let main_contract_addr = instantiate_main_contract(
        &mut app,
        main_contract_id,
        money_cw20_code_id,
        fuel_cw20_code_id,
        spaceship_cw721_code_id,
        ADDR1,
        ADDR1,
        "main contract",
    );

    let cw20_bullet_contract_addr = instantiate_cw20_contract(
        &mut app,
        bullet_cw20_code_id,
        ADDR1,
        main_contract_addr.as_ref(),
        "bullet",
        "ubullet",
        vec![],
        Uint128::new(2),
        "cw20-helper bullet",
    );

    execute_main_all_msg(
        &mut app,
        main_contract_addr.as_ref(),
        vec![FreightParams {
            contract_addr: cw20_bullet_contract_addr.to_string(),
            amount: Uint128::new(100),
        }],
        ADDR1,
        ADDR2,
    ).check_answer(
        "3",
        "8",
        "/workspace/cargo-projects/cosm-base/answers/lesson3/chapter8/execute_result.json",
    ).print_serialized();
    // .write_answer_to_file(
    //         "3",
    //         "8",
    //         "../../../answers/lesson3/chapter8/execute_result.json",
    //     );

    query_all_main_contract_msgs(&app, &main_contract_addr, ADDR1)
        .check_answer(
            "3",
            "8",
            "/workspace/cargo-projects/cosm-base/answers/lesson3/chapter8/query_result.json",
        ).print_serialized();
    // .write_answer_to_file(
    //         "",
    //         "8",
    //         "../../../answers/lesson2/chapter8/query_result.json",
    //     );
}
