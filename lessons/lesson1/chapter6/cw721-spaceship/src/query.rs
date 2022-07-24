use base::query::query_contract;
use base::result::QueryAllResult;
use cosmonaut_cw721::state::Extension;
use cosmwasm_std::{Addr, StdError};
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OwnerOfResponse, TokensResponse,
};
use cw721_base::msg::QueryMsg;
use cw721_base::MinterResponse;
use cw_multi_test::BasicApp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryResponse {
    AllNftInfoResponse(AllNftInfoResponse<Extension>),
    ContractInfoResponse(ContractInfoResponse),
    NftInfoResponse(NftInfoResponse<Extension>),
    NumTokensResponse(NumTokensResponse),
    OwnerOfResponse(OwnerOfResponse),
    TokensResponse(TokensResponse),
    MinterResponse(MinterResponse),
    ApprovalResponse(ApprovalResponse),
    ApprovalsResponse(ApprovalsResponse),
}

fn create_all_query_msgs(owner: &str) -> Vec<QueryMsg> {
    let minter_query_msg = QueryMsg::Minter {};

    let nft_info_query_msg = QueryMsg::NftInfo {
        token_id: "1".to_string(),
    };

    let approval_query_msg = QueryMsg::Approval {
        token_id: "1".to_string(),
        spender: owner.to_string(),
        include_expired: None,
    };

    let approvals_query_msg = QueryMsg::Approvals {
        token_id: "1".to_string(),
        include_expired: None,
    };

    let all_nft_info_query_msg = QueryMsg::AllNftInfo {
        token_id: "1".to_string(),
        include_expired: Some(true),
    };

    let owner_of_query_msg = QueryMsg::OwnerOf {
        token_id: "1".to_string(),
        include_expired: Some(true),
    };

    let num_tokens_msg = QueryMsg::NumTokens {};

    let tokens_msg = QueryMsg::Tokens {
        owner: owner.to_string(),
        start_after: None,
        limit: Some(30),
    };

    let contract_info_msg = QueryMsg::ContractInfo {};

    vec![
        minter_query_msg,
        approval_query_msg,
        approvals_query_msg,
        nft_info_query_msg,
        all_nft_info_query_msg,
        owner_of_query_msg,
        num_tokens_msg,
        tokens_msg,
        contract_info_msg,
    ]
}

pub fn query_all_cw721_msgs(
    app: &BasicApp,
    contract_addr: &Addr,
    owner: &str,
    recipient: &str,
) -> QueryAllResult<QueryResponse> {
    let mut responses: Vec<QueryResponse> = vec![];
    let mut errors: Vec<String> = vec![];

    let cw721_query_msgs = create_all_query_msgs(owner);

    for msg in cw721_query_msgs {
        match msg {
            QueryMsg::Minter {} => {
                let res: Result<MinterResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::Minter {});
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::MinterResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => {
                let res: Result<ApprovalResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::Approval {
                        token_id,
                        spender,
                        include_expired,
                    },
                );
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::ApprovalResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => {
                let res: Result<ApprovalsResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::Approvals {
                        token_id,
                        include_expired,
                    },
                );
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::ApprovalsResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                let res: Result<OwnerOfResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::OwnerOf {
                        token_id,
                        include_expired,
                    },
                );
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::OwnerOfResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::NftInfo { token_id } => {
                let res: Result<NftInfoResponse<Extension>, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::NftInfo { token_id });
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::NftInfoResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => {
                let res: Result<AllNftInfoResponse<Extension>, StdError> = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::AllNftInfo {
                        token_id,
                        include_expired,
                    },
                );
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::AllNftInfoResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::NumTokens {} => {
                let res: Result<NumTokensResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::NumTokens {});
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::NumTokensResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::Tokens {
                owner: _,
                start_after,
                limit,
            } => {
                let res: Result<TokensResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::Tokens {
                        owner: recipient.to_string(),
                        start_after,
                        limit,
                    },
                );
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::TokensResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            QueryMsg::ContractInfo {} => {
                let res: Result<ContractInfoResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::ContractInfo {});
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::ContractInfoResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
            _ => {}
        }
    }

    QueryAllResult { responses, errors }
}
