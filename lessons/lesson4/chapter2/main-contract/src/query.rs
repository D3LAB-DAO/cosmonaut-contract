use base::query::query_contract;
use base::result::QueryAllResult;
use cosmonaut_cw721::state::Extension;
use cosmonaut_main::msg::{ConfigResponse, FreightTokenBalanceResponse, QueryMsg};
use cosmwasm_std::{Addr, StdError};
use cw20::BalanceResponse;
use cw721::{NftInfoResponse, OwnerOfResponse};
use cw_multi_test::BasicApp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryResponse {
    ConfigResponse(ConfigResponse),
    MoneyBalanceResponse(BalanceResponse),
    OwnerOfResponse(OwnerOfResponse),
    FreightTokenBalanceResponse(FreightTokenBalanceResponse),
    FuelBalanceResponse(BalanceResponse),
    SpaceShipInfoResponse(NftInfoResponse<Extension>),
}

fn create_all_query_msgs(owner: &str) -> Vec<QueryMsg> {
    let money_token_balance_msg = QueryMsg::MoneyBalance {
        address: owner.to_string(),
    };

    let owner_of_spaceship_msg = QueryMsg::OwnerOfSpaceShip {
        token_id: "1".to_string(),
    };

    let freight_token_balance_msg = QueryMsg::FreightTokenBalance {
        symbol: "ubullet".to_string(),
        address: owner.to_string(),
    };

    let fuel_balance_msg = QueryMsg::FuelBalance {
        address: owner.to_string(),
    };

    let spaceship_info_msg = QueryMsg::SpaceShipInfo {
        token_id: "1".to_string(),
    };

    let config_msg = QueryMsg::Config {};

    vec![
        money_token_balance_msg,
        owner_of_spaceship_msg,
        freight_token_balance_msg,
        fuel_balance_msg,
        spaceship_info_msg,
        config_msg,
    ]
}

pub fn query_all_main_contract_msgs(
    app: &BasicApp,
    contract_addr: &Addr,
    owner: &str,
) -> QueryAllResult<QueryResponse> {
    let mut responses: Vec<QueryResponse> = vec![];
    let mut errors: Vec<String> = vec![];

    let main_query_msgs = create_all_query_msgs(owner);

    for msg in main_query_msgs {
        match msg {
            QueryMsg::MoneyBalance { address } => {
                let res: Result<BalanceResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::MoneyBalance { address });
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::MoneyBalanceResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }

            QueryMsg::OwnerOfSpaceShip { token_id } => {
                let res: Result<OwnerOfResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::OwnerOfSpaceShip { token_id });
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::OwnerOfResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }

            QueryMsg::FreightTokenBalance { symbol, address } => {
                let res: Result<FreightTokenBalanceResponse, StdError> = query_contract(
                    app,
                    contract_addr,
                    &QueryMsg::FreightTokenBalance { symbol, address },
                );
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::FreightTokenBalanceResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }

            QueryMsg::FuelBalance { address } => {
                let res: Result<BalanceResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::FuelBalance { address });
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::FuelBalanceResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }

            QueryMsg::SpaceShipInfo { token_id } => {
                let res: Result<NftInfoResponse<cosmonaut_cw721::state::Extension>, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::SpaceShipInfo { token_id });
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::SpaceShipInfoResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }

            QueryMsg::Config {} => {
                let res: Result<ConfigResponse, StdError> =
                    query_contract(app, contract_addr, &QueryMsg::Config {});
                match res {
                    Ok(res) => {
                        responses.push(QueryResponse::ConfigResponse(res));
                    }
                    Err(err) => {
                        errors.push(err.to_string());
                    }
                }
            }
        }
    }
    QueryAllResult { responses, errors }
}
