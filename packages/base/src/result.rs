use cosmwasm_std::Addr;
use cw_multi_test::{AppResponse, BasicApp};

pub struct InstantiateResult {
    pub app: BasicApp,
    pub addr: Addr,
}

pub struct ExecuteResult {
    pub app: BasicApp,
    pub app_response: AppResponse,
}
