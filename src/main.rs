use std::env::args;
use base::init::init_app;
use base::result::Result;

use cw20_money::init::mock_cw20_contract;
use cw20_money::instantiate::instantiate_cw20_money_contract;

use cw721_spaceship::execute::execute_cw721_all_msg;
use cw721_spaceship::init::mock_cw721_contract;
use cw721_spaceship::instantiate::instantiate_spaceship_nft_contract;
use cw721_spaceship::query::query_all_cw721_msgs;

use main_contract::init::mock_main_contract;

const ADDR1: &str = "wasm111rvne2lz6funpxxkk6yumc8ll4l3c2e3kk111";
const ADDR2: &str = "wasm222rvne2lz6funpxxkk6yumc8ll4l3c2e3kk222";
const ADDR3: &str = "wasm333rvne2lz6funpxxkk6yumc8ll4l3c2e3kk333";

const DEFAULT_DIR: &str = "./output.json";

fn main() {
    let target_dir = &args().nth(1).unwrap_or(DEFAULT_DIR.to_string());

    let mut app = init_app(ADDR1);

    let cw20_code_id = app.store_code(mock_cw20_contract());
    let cw721_code_id = app.store_code(mock_cw721_contract());
    let _main_contract_id = app.store_code(mock_main_contract());

    let cw721_spaceship_instantiate_res =
        instantiate_spaceship_nft_contract(app, cw721_code_id, ADDR1, ADDR1, "cw721 nft");
    app = cw721_spaceship_instantiate_res.app;

    let cw20_money_instantiate_res =
        instantiate_cw20_money_contract(app, cw20_code_id, ADDR1, ADDR1, "cw20 money");
    app = cw20_money_instantiate_res.app;

    let cw721_contract_addr = cw721_spaceship_instantiate_res.addr;
    let _cw20_contract_addr = cw20_money_instantiate_res.addr;

    let execute_cw721_all_results = execute_cw721_all_msg(app, cw721_contract_addr.as_ref(), ADDR1, ADDR2, ADDR3);
    execute_cw721_all_results.write_to_file(&target_dir);
    app = execute_cw721_all_results.app;

    query_all_cw721_msgs(app, &cw721_contract_addr).write_to_file(&target_dir);
}
