#[cfg(test)]
mod tests {
    use crate::contract::{instantiate, execute};
    use crate::msg::{ContractInitInfo, ExecuteMsg, InstantiateMsg};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, Addr, DepsMut};

    const ADDR1: &str = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    // const addr2: &str = "cosmos18zfp9u7zxg3gel4r3txa2jqxme7jkw7dnvfjc8";

    fn setup(deps: DepsMut) {
        let instantiate_msg = InstantiateMsg {
            money_cw20_contract: ContractInitInfo {
                addr: None,
                code_id: 1,
            },
            spaceship_cw721_contract: ContractInitInfo {
                addr: None,
                code_id: 2,
            },
        };
        let instantiate_res = instantiate(deps, mock_env(), mock_info(ADDR1, &[]), instantiate_msg).unwrap();

        assert_eq!(
            instantiate_res.attributes,
            [attr("action", "instantiate"), attr("sender", ADDR1)]
        );
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        setup(deps.as_mut());
    }

    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies();
        setup(deps.as_mut());
        let execute_buy_msg = ExecuteMsg::BuyNft {
            original_owner: ADDR1.to_string(),
            nft_id: "1".to_string(),
        };

        let execute_buy_res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADDR1, &[]), execute_buy_msg).unwrap();

        assert_eq!(
            execute_buy_res.attributes,
            [attr("", "")]
        )
    }
}
