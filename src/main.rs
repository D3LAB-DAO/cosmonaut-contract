use std::fmt::Debug;
use serde::de::DeserializeOwned;
use serde::{Serialize};

use cosmwasm_std::{coin, from_binary, Addr, BlockInfo, Empty, Coin};
use cw_multi_test::{custom_app, BasicApp, Contract, ContractWrapper, Executor, AppResponse};

const ADDR1: &str = "wasm111rvne2lz6funpxxkk6yumc8ll4l3c2e3kk111";
const ADDR2: &str = "wasm222rvne2lz6funpxxkk6yumc8ll4l3c2e3kk222";

struct InstantiateResult {
    app: BasicApp,
    addr: Addr,
}

struct ExecuteResult {
    app: BasicApp,
    app_response: AppResponse,
}

fn mock_cw20_contract() -> Box<dyn Contract<Empty>> {
    use cosmonaut_cw20::contract::{execute, instantiate, query};
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

fn mock_cw721_contract() -> Box<dyn Contract<Empty>> {
    use cosmonaut_cw721::contract::{execute, instantiate, query};
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

fn mock_main_contract() -> Box<dyn Contract<Empty>> {
    use cosmonaut_main::contract::{execute, instantiate, query, reply};
    let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    Box::new(contract)
}

fn next_block(block: &mut BlockInfo) {
    block.time = block.time.plus_seconds(5);
    block.height += 1;
}

fn init_app() -> BasicApp {
    let init_funds = vec![coin(5000, "uatom")];
    let app = custom_app::<Empty, Empty, _>(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(ADDR1), init_funds)
            .unwrap();
    });
    app
}

fn instantiate_contract<T>(
    mut app: BasicApp,
    msg: T,
    code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> InstantiateResult
    where
        T: Serialize + DeserializeOwned + Clone,
{
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(sender),
            &msg,
            &[],
            label,
            Option::from(admin.to_string()),
        )
        .unwrap();
    InstantiateResult {
        app,
        addr: contract_addr,
    }
}

fn instantiate_spaceship_nft_contract(
    app: BasicApp,
    code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> InstantiateResult {
    use cw721_base::InstantiateMsg;
    let cw721_init_msg = InstantiateMsg {
        name: "spaceship".to_string(),
        symbol: "space".to_string(),
        minter: admin.to_string(),
    };
    instantiate_contract::<InstantiateMsg>(app, cw721_init_msg, code_id, &sender, &admin, &label)
}

fn instantiate_cw20_money_contract(
    app: BasicApp,
    code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> InstantiateResult {
    use cosmonaut_cw20::msg::{InstantiateMsg, MinterResponse};

    let cw20_init_msg = InstantiateMsg {
        name: "mars".to_string(),
        symbol: "umars".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Option::from(MinterResponse {
            minter: admin.to_string(),
            cap: None,
        }),
        marketing: None,
        total_supply: None,
    };
    instantiate_contract::<InstantiateMsg>(app, cw20_init_msg, code_id, &sender, &admin, &label)
}

fn execute_contract<T>(
    mut app: BasicApp,
    contract_addr: &Addr,
    msg: T,
    send_funds: &Vec<Coin>,
) -> ExecuteResult
    where
        T: Serialize + DeserializeOwned + Clone + Debug {
    let execute_res = app.execute_contract(
        Addr::unchecked(ADDR1),
        Addr::unchecked(contract_addr),
        &msg,
        send_funds,
    ).unwrap();

    ExecuteResult {
        app,
        app_response: execute_res,
    }
}

use cosmonaut_cw721::state::Extension as cosmonautExtension;

fn create_cw721_execute_msgs() -> Vec<cosmonaut_cw721::msg::ExecuteMsg<cosmonautExtension>> {
    use cosmonaut_cw721::msg::ExecuteMsg;
    use cosmonaut_cw721::state::{Extension, Metadata};
    use cw721_base::MintMsg;

    let mint_msg = ExecuteMsg::<Extension>::Mint(MintMsg {
        token_id: "1".to_string(),
        owner: ADDR1.to_string(),
        token_uri: None,
        extension: Option::from(Metadata {
            unit_denom: "mars".to_string(),
            price: 500,
            name: Option::from("cosmonaut spaceship".to_string()),
            freight: vec![],
            health: 10,
        }),
    });

    let transfer_nft_msg = ExecuteMsg::<Extension>::TransferNft {
        recipient: ADDR2.to_string(),
        token_id: "1".to_string(),
    };

    vec![
        mint_msg,
        transfer_nft_msg,
    ]
}

fn execute_cw721_all_msg(
    mut app: BasicApp,
    contract_addr: Addr,
) -> BasicApp {
    use cosmonaut_cw721::msg::ExecuteMsg;

    let cw721_execute_msgs = create_cw721_execute_msgs();
    for msg in cw721_execute_msgs {
        let execute_res = execute_contract::<ExecuteMsg<cosmonautExtension>>(app, &contract_addr, msg, &vec![]);
        for attr in execute_res.app_response.events {
            println!("{:?}", attr.attributes);
        };
        println!();
        app = execute_res.app
    }
    app
}

fn main() {
    let mut app = init_app();

    let cw20_code_id = app.store_code(mock_cw20_contract());
    let cw721_code_id = app.store_code(mock_cw721_contract());
    let _main_contract_id = app.store_code(mock_main_contract());

    let cw721_spaceship_instantiate_res = instantiate_spaceship_nft_contract(
        app,
        cw721_code_id,
        ADDR1,
        ADDR1,
        "cw721 nft",
    );
    app = cw721_spaceship_instantiate_res.app;

    let cw20_money_instantiate_res = instantiate_cw20_money_contract(
        app,
        cw20_code_id,
        ADDR1,
        ADDR1,
        "cw20 money",
    );
    app = cw20_money_instantiate_res.app;


    let cw721_contract_addr = cw721_spaceship_instantiate_res.addr;
    let _cw20_contract_addr = cw20_money_instantiate_res.addr;

    app = execute_cw721_all_msg(app, cw721_contract_addr);
}
