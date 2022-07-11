#[cfg(test)]
mod instantiate {
    use cosmonaut_cw20::contract::instantiate;
    use cosmonaut_cw20::msg::{InstantiateMsg, MinterResponse};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, Uint128};
    use cw20::Cw20Coin;

    #[test]
    fn test_instantiate() {
        let addr1 = "osmo18zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let mut deps = mock_dependencies();
        let _env = mock_env();
        let info = mock_info(addr1, &[]);

        let mint_msg = MinterResponse {
            minter: addr1.to_string(),
            cap: Some(Uint128::new(100)),
        };

        let msg = InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            decimals: 8,
            initial_balances: vec![Cw20Coin {
                address: addr1.to_string(),
                amount: Uint128::new(100),
            }],
            mint: Some(mint_msg),
            marketing: None,
            total_supply: Uint128::zero(),
            unit_weight: None,
        };

        let res = instantiate(deps.as_mut(), _env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("action", "instantiate"),
                attr("minter_address", addr1.to_string()),
                attr("minter_cap", "100".to_string()),
            ]
        );
    }
}
