use base::init::init_app;
use base::result::Result;
use std::env::args;

use cw20_money::init::mock_cw20_contract;
use cw20_money::instantiate::instantiate_cw20_money_contract;

use cw721_spaceship::execute::execute_cw721_all_msg;
use cw721_spaceship::init::mock_cw721_contract;
use cw721_spaceship::instantiate::instantiate_spaceship_nft_contract;
use cw721_spaceship::query::query_all_cw721_msgs;
use main_contract::execute::execute_main_all_msg;

use main_contract::init::mock_main_contract;
use main_contract::instantiate::instantiate_main_contract;

const ADDR1: &str = "wasm111rvne2lz6funpxxkk6yumc8ll4l3c2e3kk111";
const ADDR2: &str = "wasm222rvne2lz6funpxxkk6yumc8ll4l3c2e3kk222";
const ADDR3: &str = "wasm333rvne2lz6funpxxkk6yumc8ll4l3c2e3kk333";

const DEFAULT_ANSWER_PATH: &str = "answers";

fn main() {
    if args().count() != 4 {
        println!("args are not sufficient");
        std::process::exit(0);
    }

    let execute_output_dir: &str = &args().nth(1).unwrap();
    let query_output_dir: &str = &args().nth(2).unwrap();
    let which_lesson: &str = &args().nth(3).unwrap();

    let mut app = init_app(ADDR1);

    let cw20_code_id = app.store_code(mock_cw20_contract());
    let cw721_code_id = app.store_code(mock_cw721_contract());
    let main_contract_id = app.store_code(mock_main_contract());

    let cw721_contract_addr =
        instantiate_spaceship_nft_contract(&mut app, cw721_code_id, ADDR1, ADDR1, "cw721 nft");
    let _cw20_contract_addr =
        instantiate_cw20_money_contract(&mut app, cw20_code_id, ADDR1, ADDR1, "cw20 money");
    let main_contract_addr = instantiate_main_contract(
        &mut app,
        main_contract_id,
        cw20_code_id,
        cw721_code_id,
        ADDR1,
        ADDR1,
        "main contract",
    );

    execute_cw721_all_msg(&mut app, cw721_contract_addr.as_ref(), ADDR1, ADDR2, ADDR3)
        .check_answer(
            which_lesson,
            &format!(
                "./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_execute_result.json",
                which_lesson, which_lesson
            ),
        )
        .write_to_file(execute_output_dir);
    query_all_cw721_msgs(&app, &cw721_contract_addr, ADDR1, ADDR2)
        .check_answer(
            which_lesson,
            &format!(
                "./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_query_result.json",
                which_lesson, which_lesson
            ),
        )
        .write_to_file(query_output_dir);

    execute_main_all_msg(&mut app, main_contract_addr.as_ref(), [], ADDR1, ADDR2)
        .check_answer(
            which_lesson,
            &format!(
                "./{DEFAULT_ANSWER_PATH}/lesson{}/lesson{}_execute_result.json",
                which_lesson, which_lesson
            ),
        )
        .write_to_file(execute_output_dir)
}
