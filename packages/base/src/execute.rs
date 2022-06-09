use cosmwasm_std::{Addr, Attribute, Coin};
use cw_multi_test::{BasicApp, Executor};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub fn execute_contract<T>(
    app: &mut BasicApp,
    contract_addr: &Addr,
    msg: &T,
    send_funds: &[Coin],
    sender: &str,
) -> Vec<Attribute>
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

    execute_res.events[1].clone().attributes
}
