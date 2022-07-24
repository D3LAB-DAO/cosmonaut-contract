use base::consts::*;
use base::init::init_app;
use base::result::Result;
use cw721_spaceship::execute::execute_cw721_all_msg;
use cw721_spaceship::init::mock_cw721_contract;
use cw721_spaceship::instantiate::instantiate_spaceship_nft_contract;
use cw721_spaceship::query::query_all_cw721_msgs;

pub fn main() {
    let mut app = init_app(ADDR1);

    let spaceship_cw721_code_id = app.store_code(mock_cw721_contract());
    let cw721_contract_addr = instantiate_spaceship_nft_contract(
        &mut app,
        spaceship_cw721_code_id,
        ADDR1,
        ADDR1,
        "cw721 nft",
    );

    execute_cw721_all_msg(&mut app, cw721_contract_addr.as_ref(), ADDR1, ADDR2, ADDR3)
        .check_answer(
            "1",
            "6",
            "../../../answers/lesson1/chapter6/execute_result.json",
        ).print_serialized();

    // .write_answer_to_file(
    //     "1",
    //     "6",
    //     "../../../answers/lesson1/chapter6/execute_result.json"
    // );

    query_all_cw721_msgs(&app, &cw721_contract_addr, ADDR1, ADDR2)
        .check_answer(
            "1",
            "6",
            "../../../answers/lesson1/chapter6/query_result.json",
        ).print_serialized();
    // .write_answer_to_file(
    //     "1",
    //     "6",
    //     "../../../answers/lesson1/chapter6/query_result.json"
    // );
}
