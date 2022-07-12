use base::query::query_contract;
use base::result::QueryAllResult;
use cosmwasm_std::{Addr, StdError};
use cw20::{AllAccountsResponse, AllAllowancesResponse, AllowanceResponse, BalanceResponse, Cw20QueryMsg, MinterResponse, TokenInfoResponse};
use cw_multi_test::BasicApp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryResponse {
    MinterResponse(MinterResponse),
    AllowanceResponse(AllowanceResponse),
    BalanceResponse(BalanceResponse),
    TokenInfoResponse(TokenInfoResponse),
    AllAllowancesResponse(AllAllowancesResponse),
    AllAccountsResponse(AllAccountsResponse),
}

fn create_all_query_msgs(owner: &str, spender: &str) -> Vec<Cw20QueryMsg> {
    let minter_query_msg = Cw20QueryMsg::Minter {};
    let allowance_query_msg = Cw20QueryMsg::Allowance {
        owner: owner.to_string(),
        spender: spender.to_string(),
    };
    let balance_query_msg = Cw20QueryMsg::Balance {
        address: owner.to_string(),
    };
    let token_info_query_msg = Cw20QueryMsg::TokenInfo {};
    let all_allowances_query_msg = Cw20QueryMsg::AllAllowances {
        owner: owner.to_string(),
        start_after: None,
        limit: None,
    };
    let all_accounts_query_msg = Cw20QueryMsg::AllAccounts {
        start_after: None,
        limit: None,
    };

    vec![
        minter_query_msg,
        allowance_query_msg,
        balance_query_msg,
        token_info_query_msg,
        all_allowances_query_msg,
        all_accounts_query_msg,
    ]
}

pub fn query_all_cw20_msgs(
    app: &BasicApp,
    contract_addr: &Addr,
    owner: &str,
    recipient: &str,
) -> QueryAllResult<QueryResponse> {
    let cw20_query_msgs = create_all_query_msgs(owner, recipient);

    let mut responses: Vec<QueryResponse> = vec![];
    let mut errors: Vec<String> = vec![];

    for msg in cw20_query_msgs {
        match msg {
            Cw20QueryMsg::Minter {} => {
                let res: Result<MinterResponse, StdError> =
                    query_contract(app, contract_addr, &Cw20QueryMsg::Minter {});
                match res {
                    Ok(res) => responses.push(QueryResponse::MinterResponse(res)),
                    Err(err) => errors.push(err.to_string()),
                }
            }

            Cw20QueryMsg::Allowance { owner, spender } => {
                let res: Result<AllowanceResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &Cw20QueryMsg::Allowance { owner, spender },
                );
                match res {
                    Ok(res) => responses.push(QueryResponse::AllowanceResponse(res)),
                    Err(err) => errors.push(err.to_string()),
                }
            }

            Cw20QueryMsg::Balance { address } => {
                let res: Result<BalanceResponse, StdError> =
                    query_contract(app, contract_addr, &Cw20QueryMsg::Balance { address });
                match res {
                    Ok(res) => responses.push(QueryResponse::BalanceResponse(res)),
                    Err(err) => errors.push(err.to_string()),
                }
            }

            Cw20QueryMsg::TokenInfo {} => {
                let res: Result<TokenInfoResponse, StdError> =
                    query_contract(app, contract_addr, &Cw20QueryMsg::TokenInfo {});
                match res {
                    Ok(res) => responses.push(QueryResponse::TokenInfoResponse(res)),
                    Err(err) => errors.push(err.to_string()),
                }
            }

            Cw20QueryMsg::AllAllowances {
                owner,
                start_after,
                limit,
            } => {
                let res: Result<AllAllowancesResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &Cw20QueryMsg::AllAllowances {
                        owner,
                        start_after,
                        limit,
                    },
                );
                match res {
                    Ok(res) => responses.push(QueryResponse::AllAllowancesResponse(res)),
                    Err(err) => errors.push(err.to_string()),
                }
            }

            Cw20QueryMsg::AllAccounts { start_after, limit } => {
                let res: Result<AllAccountsResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &Cw20QueryMsg::AllAccounts { start_after, limit },
                );
                match res {
                    Ok(res) => responses.push(QueryResponse::AllAccountsResponse(res)),
                    Err(err) => errors.push(err.to_string()),
                }
            }
            _ => {}
        }
    }

    QueryAllResult { responses, errors }
}
