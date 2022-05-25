#[cfg(test)]
mod tests {
    use crate::contract::{instantiate};
    use crate::msg::{ContractInitInfo, InstantiateMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, Addr, DepsMut};

    const CW20_CONTRACT: &str = "wasmcw20p9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const CW721_CONTRACT: &str = "wasmcw7219u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const ADDR1: &str = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    // const addr2: &str = "cosmos18zfp9u7zxg3gel4r3txa2jqxme7jkw7dnvfjc8";

    fn setup(deps: DepsMut) {
        let instantiate_msg = InstantiateMsg {
            money_cw20_contract: ContractInitInfo {
                addr: Addr::unchecked(CW20_CONTRACT),
                code_id: 1,
            },
            spaceship_cw721_contract: ContractInitInfo {
                addr: Addr::unchecked(CW721_CONTRACT),
                code_id: 2,
            },
        };
        let instantiate_res =
            instantiate(deps, mock_env(), mock_info(ADDR1, &[]), instantiate_msg).unwrap();

        assert_eq!(
            instantiate_res.attributes,
            [
                attr("action", "instantiate"),
                attr("sender", ADDR1),
                attr("cw20_address", CW20_CONTRACT),
                attr("cw721_address", CW721_CONTRACT)
            ]
        );
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        setup(deps.as_mut())
    }

    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies();
        setup(deps.as_mut())
    }
}
