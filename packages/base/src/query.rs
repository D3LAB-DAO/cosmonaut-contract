use cosmwasm_std::Addr;
use cw_multi_test::BasicApp;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub fn query_contract<T, Q>(app: &BasicApp, contract_addr: &Addr, msg: &T) -> Q
where
    T: Serialize + DeserializeOwned + Clone + Debug,
    Q: DeserializeOwned,
{
    app.wrap().query_wasm_smart(contract_addr, msg).unwrap()
}
