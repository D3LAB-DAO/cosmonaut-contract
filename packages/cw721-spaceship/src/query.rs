use base::query::query_contract;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::Addr;
use cw721::{NftInfoResponse, NumTokensResponse, OwnerOfResponse};
use cw721_base::msg::QueryMsg;
use cw_multi_test::BasicApp;

fn create_all_query_msgs() -> Vec<QueryMsg> {
    let nft_info_query_msg = QueryMsg::NftInfo {
        token_id: "1".to_string(),
    };

    let owner_of_query_msg = QueryMsg::OwnerOf {
        token_id: "1".to_string(),
        include_expired: Option::from(true),
    };

    let num_tokens_msg = QueryMsg::NumTokens {};

    vec![
        nft_info_query_msg,
        owner_of_query_msg,
        num_tokens_msg,
    ]
}

pub fn query_all_cw721_msgs(
    app: BasicApp,
    contract_addr: &Addr,
) -> BasicApp {
    let cw721_query_msgs = create_all_query_msgs();
    for msg in cw721_query_msgs {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                let res: OwnerOfResponse = query_contract(
                    &app,
                    contract_addr,
                    &QueryMsg::OwnerOf {
                        token_id,
                        include_expired,
                    },
                );
                println!("{:?}", res);
            }
            QueryMsg::NftInfo { token_id } => {
                let res: NftInfoResponse<Extension> =
                    query_contract(&app, contract_addr, &QueryMsg::NftInfo { token_id });
                println!("{:?}", res);
            }
            QueryMsg::NumTokens {} => {
                let res: NumTokensResponse = query_contract(
                    &app,
                    contract_addr,
                    &QueryMsg::NumTokens {},
                );
                println!("{:?}", res);
            }
            _ => {}
        }
    }

    app
}
