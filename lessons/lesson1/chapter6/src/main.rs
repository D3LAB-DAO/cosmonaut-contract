use base::consts::*;
use base::init::init_app;
use base::result::Result;
use cw721_spaceship::execute::execute_cw721_all_msg;
use cw721_spaceship::init::mock_cw721_contract;
use cw721_spaceship::instantiate::instantiate_spaceship_nft_contract;
use cw721_spaceship::query::query_all_cw721_msgs;
use std::env;
use std::env::args;

pub fn main() {
    if args().count() != 3 {
        println!("args are not sufficient");
        std::process::exit(0);
    }

    let which_lesson: &str = &args().nth(1).unwrap();
    let which_chapter: &str = &args().nth(2).unwrap();
    // let volume_path: &str = &env::var("BASE_VOLUME_DIR").unwrap();
    let volume_path: &str = "/Users/ogsang-yun/Documents/IdeaProjects/cosmonaut-contract";

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
            which_lesson,
            which_chapter,
            &format!(
                "{}/answers/lesson{}/chapter{}/execute_result.json",
                volume_path, which_lesson, which_chapter,
            ),
        ).print_serialized();

    // .write_answer_to_file(&format!(
    //     "{}/answers/lesson{}/chapter{}/execute_result.json",
    //     volume_path, which_lesson, which_chapter,
    // ));

    query_all_cw721_msgs(&app, &cw721_contract_addr, ADDR1, ADDR2)
        .check_answer(
            which_lesson,
            which_chapter,
            &format!(
                "{}/answers/lesson{}/chapter{}/query_result.json",
                volume_path, which_lesson, which_chapter,
            ),
        ).print_serialized();
    // .write_answer_to_file(&format!(
    //     "{}/answers/lesson{}/chapter{}/query_result.json",
    //     volume_path, which_lesson, which_chapter,
    // ));
}
