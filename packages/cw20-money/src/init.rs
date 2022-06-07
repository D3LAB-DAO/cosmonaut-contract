use cosmwasm_std::{Empty};
use cw_multi_test::{Contract, ContractWrapper};

pub fn mock_cw20_contract() -> Box<dyn Contract<Empty>> {
    use cosmonaut_cw20::contract::{execute, instantiate, query};
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
