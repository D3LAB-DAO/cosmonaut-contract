use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};
use cosmonaut_cw721::contract::{execute, instantiate, query};

pub fn mock_cw721_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
