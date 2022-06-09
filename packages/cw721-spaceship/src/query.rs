use base::query::query_contract;
use base::result::QueryAllResult;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::{Addr};
use cw721::{NftInfoResponse, NumTokensResponse, OwnerOfResponse, TokensResponse, ContractInfoResponse};
use cw721_base::msg::QueryMsg;
use cw_multi_test::BasicApp;

fn create_all_query_msgs(owner: &str) -> Vec<QueryMsg> {
    let nft_info_query_msg = QueryMsg::NftInfo {
        token_id: "1".to_string(),
    };

    let owner_of_query_msg = QueryMsg::OwnerOf {
        token_id: "1".to_string(),
        include_expired: Option::from(true),
    };

    let num_tokens_msg = QueryMsg::NumTokens {};

    let tokens_msg = QueryMsg::Tokens {
        owner: owner.to_string(),
        start_after: None,
        limit: Option::from(30),
    };

    let contract_info_msg = QueryMsg::ContractInfo {};

    vec![
        nft_info_query_msg,
        owner_of_query_msg,
        num_tokens_msg,
        tokens_msg,
        contract_info_msg,
    ]
}

pub fn query_all_cw721_msgs(app: &BasicApp, contract_addr: &Addr, owner: &str, recipient: &str) -> QueryAllResult {
    let cw721_query_msgs = create_all_query_msgs(owner);
    let mut query_results: Vec<String> = vec![];

    for msg in cw721_query_msgs {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                let res: OwnerOfResponse = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::OwnerOf {
                        token_id,
                        include_expired,
                    },
                );
                query_results.push(serde_json::to_string(&res).unwrap());
            }
            QueryMsg::NftInfo { token_id } => {
                let res: NftInfoResponse<Extension> =
                    query_contract(app, contract_addr, &QueryMsg::NftInfo { token_id });
                query_results.push(serde_json::to_string(&res).unwrap());
            }
            QueryMsg::NumTokens {} => {
                let res: NumTokensResponse =
                    query_contract(app, contract_addr, &QueryMsg::NumTokens {});
                query_results.push(serde_json::to_string(&res).unwrap());
            }
            QueryMsg::Tokens {
                owner: _,
                start_after,
                limit,
            } => {
                let res: TokensResponse = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::Tokens {
                        owner: recipient.to_string(),
                        start_after,
                        limit,
                    },
                );
                query_results.push(serde_json::to_string(&res).unwrap());
            }
            QueryMsg::ContractInfo {} => {
                let res: ContractInfoResponse = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::ContractInfo {},
                );
                query_results.push(serde_json::to_string(&res).unwrap());
            }
            _ => {}
        }
    }

    QueryAllResult { query_results }
}
