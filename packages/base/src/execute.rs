use crate::result::ExecuteResult;
use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{BasicApp, Executor};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub fn execute_contract<T>(
    mut app: BasicApp,
    contract_addr: &Addr,
    msg: &T,
    send_funds: &[Coin],
    sender: &str,
) -> ExecuteResult
where
    T: Serialize + DeserializeOwned + Clone + Debug,
{
    let execute_res = app
        .execute_contract(
            Addr::unchecked(sender),
            Addr::unchecked(contract_addr),
            &msg,
            send_funds,
        )
        .unwrap();

    ExecuteResult {
        app,
        attributes: execute_res.events[1].clone().attributes,
    }
}
