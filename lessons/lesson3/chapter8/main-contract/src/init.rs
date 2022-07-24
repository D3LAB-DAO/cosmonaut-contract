use cosmwasm_std::Empty;
use cw_multi_test::{Contract, ContractWrapper};

pub fn mock_main_contract() -> Box<dyn Contract<Empty>> {
    use cosmonaut_main::contract::{execute, instantiate, query, reply};
    let contract = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    Box::new(contract)
}
