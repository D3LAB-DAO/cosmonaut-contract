use cosmwasm_std::Addr;
use cw_multi_test::{BasicApp, Executor};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::result::{InstantiateResult};

pub fn instantiate_contract<T>(
    mut app: BasicApp,
    msg: T,
    code_id: u64,
    sender: &str,
    admin: &str,
    label: &str,
) -> InstantiateResult
    where
        T: Serialize + DeserializeOwned + Clone,
{
    let contract_addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(sender),
            &msg,
            &[],
            label,
            Option::from(admin.to_string()),
        )
        .unwrap();
    InstantiateResult {
        app,
        addr: contract_addr,
    }
}
