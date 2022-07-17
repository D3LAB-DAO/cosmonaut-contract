use base::consts::*;
use base::init::init_app;
use base::result::Result;
use cw721_spaceship::execute::execute_cw721_all_msg;
use cw721_spaceship::init::mock_cw721_contract;
use cw721_spaceship::instantiate::instantiate_spaceship_nft_contract;
use cw721_spaceship::query::query_all_cw721_msgs;
use std::env::args;

pub fn main() {
    if args().count() != 2 {
        println!("args are not sufficient");
        std::process::exit(0);
    }

    let which_lesson: &str = &args().nth(1).unwrap();
    let volume_path: &str = &args().nth(2).unwrap();

    let mut app = init_app(ADDR1);

    let spaceship_cw721_code_id = app.store_code(mock_cw721_contract());
    let cw721_contract_addr = instantiate_spaceship_nft_contract(
        &mut app,
        spaceship_cw721_code_id,
        ADDR1,
        ADDR1,
        "cw721 nft",
    );

    println!(
        "{:?}",
        execute_cw721_all_msg(&mut app, cw721_contract_addr.as_ref(), ADDR1, ADDR2, ADDR3)
            .check_answer(
                which_lesson,
                volume_path,
            )
    );
    // .write_answer_to_file(&format!("./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_execute_result.json", which_lesson, which_lesson));

    println!(
        "{:?}",
        query_all_cw721_msgs(&app, &cw721_contract_addr, ADDR1, ADDR2).check_answer(
            which_lesson,
            volume_path,
        )
    );
    // .write_answer_to_file(&format!("./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_query_result.json", which_lesson, which_lesson));
}
