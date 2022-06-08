use base::query::query_contract;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::{Addr, QueryRequest};
use cw721::{ApprovalResponse, Cw721QueryMsg, NftInfoResponse, OwnerOfResponse};
use cw_multi_test::BasicApp;

pub fn query_nft_info(app: BasicApp, contract_addr: &str) -> BasicApp {
    let res: NftInfoResponse<Extension> = app
        .wrap()
        .query_wasm_smart(
            contract_addr,
            &Cw721QueryMsg::NftInfo {
                token_id: "1".to_string(),
            },
        )
        .unwrap();

    println!("{:?}", res);

    app
}

fn create_all_query_msgs() -> Vec<Cw721QueryMsg> {
    let nft_info_query_msg = Cw721QueryMsg::NftInfo {
        token_id: "1".to_string(),
    };

    let owner_of_query_msg = Cw721QueryMsg::OwnerOf {
        token_id: "1".to_string(),
        include_expired: None,
    };

    vec![nft_info_query_msg, owner_of_query_msg]
}

pub fn query_all_cw721_msgs(app: BasicApp, contract_addr: &Addr) -> BasicApp {
    let cw721_query_msgs = create_all_query_msgs();
    for msg in cw721_query_msgs {
        match msg {
            Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                let res: OwnerOfResponse = query_contract(
                    &app,
                    &contract_addr,
                    &Cw721QueryMsg::OwnerOf {
                        token_id,
                        include_expired,
                    },
                );
                println!("{:?}", res);
            }
            Cw721QueryMsg::NftInfo { token_id } => {
                let res: NftInfoResponse<Extension> =
                    query_contract(&app, &contract_addr, &Cw721QueryMsg::NftInfo { token_id });
                println!("{:?}", res);
            }
            Cw721QueryMsg::Approved { owner, operator } => {
                let res: ApprovalResponse = query_contract(
                    &app,
                    &contract_addr,
                    &Cw721QueryMsg::Approved { owner, operator },
                );
                println!("{:?}", res);
            }
            _ => {}
        }
    }

    app
}
