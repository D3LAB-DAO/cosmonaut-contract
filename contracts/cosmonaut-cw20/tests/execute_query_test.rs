#[cfg(test)]
mod execute {
    use cosmonaut_cw20::contract::{execute, instantiate, query};
    use cosmonaut_cw20::msg::ExecuteMsg::{DecreaseAllowance, IncreaseAllowance, TransferFrom};
    use cosmonaut_cw20::msg::{
        AllowanceResponse, BalanceResponse, ExecuteMsg, InstantiateMsg, MintInfoResponse,
        MinterResponse, QueryMsg, TokenInfoResponse,
    };
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, Uint128};
    use cw20::Cw20Coin;
    use cw_utils::Expiration;
    use cw_utils::Expiration::AtHeight;

    #[test]
    fn execute_transfer() {
        let addr1 = "osmo18zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let addr2 = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr1, &[]);

        let mint_msg = MinterResponse {
            minter: addr1.to_string(),
            cap: Some(Uint128::new(100)),
        };

        let _msg = InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            decimals: 8,
            initial_balances: vec![Cw20Coin {
                address: addr1.to_string(),
                amount: Uint128::new(100),
            }],
            mint: Some(mint_msg),
            marketing: None,
            total_supply: None,
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), _msg).unwrap();

        let msg = ExecuteMsg::Transfer {
            recipient: addr2.to_string(),
            amount: Uint128::new(20),
        };

        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(
            res.attributes,
            [
                attr("action", "transfer"),
                attr("sender", addr1),
                attr("recipient", addr2),
                attr("amount", "20")
            ]
        );

        let query_addr1_msg = QueryMsg::Balance {
            address: addr1.to_string(),
        };

        let query_addr1_res = query(deps.as_ref(), env.clone(), query_addr1_msg).unwrap();

        let query_addr1: BalanceResponse = from_binary(&query_addr1_res).unwrap();

        assert_eq!(
            query_addr1,
            BalanceResponse {
                balance: Uint128::new(80)
            }
        );

        let query_addr2_msg = QueryMsg::Balance {
            address: addr2.to_string(),
        };

        let query_addr2_res = query(deps.as_ref(), env, query_addr2_msg).unwrap();

        let query_addr2: BalanceResponse = from_binary(&query_addr2_res).unwrap();

        assert_eq!(
            query_addr2,
            BalanceResponse {
                balance: Uint128::new(20)
            }
        )
    }

    #[test]
    fn test_mint() {
        let addr1 = "osmo18zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let addr2 = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr1, &[]);

        let mint_spec = MinterResponse {
            minter: addr1.to_string(),
            cap: Some(Uint128::new(1000)),
        };

        let _msg = InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            decimals: 8,
            initial_balances: vec![Cw20Coin {
                address: addr1.to_string(),
                amount: Uint128::new(100),
            }],
            mint: Some(mint_spec),
            marketing: None,
            total_supply: None,
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), _msg).unwrap();

        let mint_msg = ExecuteMsg::Mint {
            recipient: addr2.to_string(),
            amount: Uint128::new(30),
        };

        let mint_res = execute(deps.as_mut(), env.clone(), info, mint_msg).unwrap();

        assert_eq!(
            mint_res.attributes,
            [
                attr("action", "mint"),
                attr("recipient", addr2),
                attr("amount", "30")
            ]
        );

        let query_token_info_msg = QueryMsg::TokenInfo {};
        let token_info_response_bin =
            query(deps.as_ref(), env.clone(), query_token_info_msg).unwrap();

        let token_info_response: TokenInfoResponse = from_binary(&token_info_response_bin).unwrap();
        assert_eq!(
            token_info_response,
            TokenInfoResponse {
                name: "mars".to_string(),
                symbol: "MARS".to_string(),
                decimals: 8,
                total_supply: Uint128::new(130),
            }
        );

        let query_mint_info_msg = QueryMsg::MintInfo {};

        let mint_info_response_bin = query(deps.as_ref(), env, query_mint_info_msg).unwrap();

        let mint_info_response = from_binary(&mint_info_response_bin).unwrap();

        assert_eq!(
            mint_info_response,
            MintInfoResponse {
                minter: addr1.to_string(),
                cap: Some(Uint128::new(1000)),
            }
        )
    }

    #[test]
    fn test_execute_increase_decrease_allowance() {
        let addr1 = "osmo18zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let addr2 = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr1, &[]);

        let mint_spec = MinterResponse {
            minter: addr1.to_string(),
            cap: Some(Uint128::new(1000)),
        };

        let _msg = InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            decimals: 8,
            initial_balances: vec![Cw20Coin {
                address: addr1.to_string(),
                amount: Uint128::new(100),
            }],
            mint: Some(mint_spec),
            marketing: None,
            total_supply: None,
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), _msg).unwrap();

        let increase_allowance_msg = IncreaseAllowance {
            spender: addr2.to_string(),
            amount: Uint128::new(30),
            expires: Some(AtHeight(env.block.height + 180 / 5)),
        };

        let increase_allowance_2_msg = IncreaseAllowance {
            spender: addr2.to_string(),
            amount: Uint128::new(20),
            expires: Some(AtHeight(env.block.height + 180 / 5)),
        };

        let increase_allowance_res = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            increase_allowance_msg,
        )
        .unwrap();

        let increase_allowance_res_2 = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            increase_allowance_2_msg,
        )
        .unwrap();

        assert_eq!(
            increase_allowance_res.attributes,
            [
                attr("action", "increase_allowance"),
                attr("owner", addr1),
                attr("spender", addr2),
                attr("amount", "30")
            ]
        );

        assert_eq!(
            increase_allowance_res_2.attributes,
            [
                attr("action", "increase_allowance"),
                attr("owner", addr1),
                attr("spender", addr2),
                attr("amount", "20")
            ]
        );

        let query_allowance_msg = QueryMsg::Allowance {
            owner: addr1.to_string(),
            spender: addr2.to_string(),
        };

        let allowance_res_bin = query(deps.as_ref(), env.clone(), query_allowance_msg).unwrap();

        let allowance_res: AllowanceResponse = from_binary(&allowance_res_bin).unwrap();
        assert_eq!(
            allowance_res,
            AllowanceResponse {
                allowance: Uint128::new(50),
                expires: Expiration::AtHeight(12381),
            }
        );

        let decrease_allowance_msg = DecreaseAllowance {
            spender: addr2.to_string(),
            amount: Uint128::new(25),
            expires: Some(AtHeight(env.block.height + 180 / 5)),
        };

        execute(deps.as_mut(), env.clone(), info, decrease_allowance_msg).unwrap();

        let query_allowance_after_msg = QueryMsg::Allowance {
            owner: addr1.to_string(),
            spender: addr2.to_string(),
        };

        let allowance_res_bin = query(deps.as_ref(), env, query_allowance_after_msg).unwrap();

        let allowance_res_after: AllowanceResponse = from_binary(&allowance_res_bin).unwrap();
        assert_eq!(
            allowance_res_after,
            AllowanceResponse {
                allowance: Uint128::new(25),
                expires: Expiration::AtHeight(12381),
            }
        );
    }

    #[test]
    fn test_transfer_from() {
        let addr1 = "osmo18zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let addr2 = "osmo17zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let addr3 = "osmo16zfp9u7zxg3gel4r3txa2jqxme7jkw7dmh6zw4";
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(addr1, &[]);

        let mint_spec = MinterResponse {
            minter: addr1.to_string(),
            cap: Some(Uint128::new(1000)),
        };

        let _msg = InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            decimals: 8,
            initial_balances: vec![Cw20Coin {
                address: addr1.to_string(),
                amount: Uint128::new(100),
            }],
            mint: Some(mint_spec),
            marketing: None,
            total_supply: None,
        };

        instantiate(deps.as_mut(), env.clone(), info.clone(), _msg).unwrap();

        let increase_allowance_msg = IncreaseAllowance {
            spender: addr2.to_string(),
            amount: Uint128::new(30),
            expires: Some(AtHeight(env.block.height + 180 / 5)),
        };

        execute(deps.as_mut(), env.clone(), info, increase_allowance_msg).unwrap();

        let query_allowance_msg = QueryMsg::Allowance {
            owner: addr1.to_string(),
            spender: addr2.to_string(),
        };

        let allowance_res_bin = query(deps.as_ref(), env.clone(), query_allowance_msg).unwrap();

        let allowance_res: AllowanceResponse = from_binary(&allowance_res_bin).unwrap();
        assert_eq!(
            allowance_res,
            AllowanceResponse {
                allowance: Uint128::new(30),
                expires: AtHeight(12381),
            }
        );

        let transfer_from_msg = TransferFrom {
            owner: addr1.to_string(),
            recipient: addr3.to_string(),
            amount: Uint128::new(15),
        };

        let info_addr = mock_info(addr2, &[]);

        execute(deps.as_mut(), env.clone(), info_addr, transfer_from_msg).unwrap();

        let query_addr1_msg = QueryMsg::Balance {
            address: addr1.to_string(),
        };

        let query_addr1_res = query(deps.as_ref(), env.clone(), query_addr1_msg).unwrap();

        let query_addr1_balance: BalanceResponse = from_binary(&query_addr1_res).unwrap();

        assert_eq!(
            query_addr1_balance,
            BalanceResponse {
                balance: Uint128::new(85)
            }
        );

        let query_addr3_msg = QueryMsg::Balance {
            address: addr3.to_string(),
        };

        let query_addr3_res = query(deps.as_ref(), env, query_addr3_msg).unwrap();

        let query_addr3_balance: BalanceResponse = from_binary(&query_addr3_res).unwrap();

        assert_eq!(
            query_addr3_balance,
            BalanceResponse {
                balance: Uint128::new(15)
            }
        )
    }
}
