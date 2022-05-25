#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, DepsMut};
    use crate::contract::instantiate;
    use crate::msg::{ContractInitInfo, InstantiateMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    const cw20_contract: &str = "wasmcw20p9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const cw721_contract: &str = "wasmcw7219u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const addr1: &str = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const addr2: &str = "cosmos18zfp9u7zxg3gel4r3txa2jqxme7jkw7dnvfjc8";

    fn setup(mut deps: DepsMut) {
        let instantiate_msg = InstantiateMsg {
            money_cw20_contract: ContractInitInfo {
                addr: cw20_contract.to_string(),
                code_id: 1,
            },
            spaceship_cw721_contract: ContractInitInfo {
                addr: cw721_contract.to_string(),
                code_id: 2,
            },
        };
        let instantiate_res = instantiate(
            deps,
            mock_env(),
            mock_info(addr1, &[]),
            instantiate_msg,
        ).unwrap();

        assert_eq!(
            instantiate_res.attributes,
            [
                attr("action", "instantiate"),
                attr("sender", addr1),
                attr("cw20_address", cw20_contract),
                attr("cw721_address", cw721_contract)
            ]
        );
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        setup(deps.as_mut())
    }
}
