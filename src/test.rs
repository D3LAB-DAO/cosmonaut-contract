#[cfg(test)]
mod tests {
    use crate::contract;
    use crate::msg::ExecuteMsg::{AddLuggageContract, LoadLuggage};
    use crate::msg::{
        ContractInitInfo, ExecuteMsg, InstantiateMsg, MoneyContractResponse, QueryMsg,
    };
    use cosmonaut_cw20::msg::ExecuteMsg::IncreaseAllowance;
    use cosmonaut_cw20::msg::{BalanceResponse, MinterResponse};
    use cosmonaut_cw721::state::{Extension, Luggage, Metadata};
    use cosmwasm_std::{coin, coins, Addr, Empty, Uint128};
    use cw20::Cw20Coin;
    use cw721::Cw721QueryMsg::NftInfo;
    use cw721::NftInfoResponse;
    use cw721_base::MintMsg;
    use cw_multi_test::{custom_app, Contract, ContractWrapper, Executor};

    const ADDR1: &str = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";

    fn mock_cw20_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cosmonaut_cw20::contract::execute,
            cosmonaut_cw20::contract::instantiate,
            cosmonaut_cw20::contract::query,
        );
        Box::new(contract)
    }

    fn mock_cw721_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cosmonaut_cw721::contract::execute,
            cosmonaut_cw721::contract::instantiate,
            cosmonaut_cw721::contract::query,
        );
        Box::new(contract)
    }

    fn mock_main_contract() -> Box<dyn Contract<Empty>> {
        let contract =
            ContractWrapper::new(contract::execute, contract::instantiate, contract::query)
                .with_reply(contract::reply);
        Box::new(contract)
    }

    #[test]
    fn test_execute() {
        let init_funds = vec![coin(100000, "atom")];
        let mut app = custom_app::<Empty, Empty, _>(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADDR1), init_funds)
                .unwrap();
        });

        let cw20_code_id = app.store_code(mock_cw20_contract());
        let cw721_code_id = app.store_code(mock_cw721_contract());
        let main_contract_id = app.store_code(mock_main_contract());

        let instantiate_msg = InstantiateMsg {
            money_cw20_contract: ContractInitInfo {
                addr: None,
                code_id: cw20_code_id,
            },
            spaceship_cw721_contract: ContractInitInfo {
                addr: None,
                code_id: cw721_code_id,
            },
        };

        let contract_addr = app
            .instantiate_contract(
                main_contract_id,
                Addr::unchecked(ADDR1),
                &instantiate_msg,
                &coins(1000, "atom"),
                "main contract",
                Option::from(ADDR1.to_string()),
            )
            .unwrap();

        let oil_cw20_contract_addr = app
            .instantiate_contract(
                cw20_code_id,
                Addr::unchecked(ADDR1),
                &cosmonaut_cw20::msg::InstantiateMsg {
                    name: "OIL".to_string(),
                    symbol: "oil".to_string(),
                    decimals: 6,
                    initial_balances: vec![Cw20Coin {
                        address: ADDR1.to_string(),
                        amount: Uint128::new(10000),
                    }],
                    mint: Option::from(MinterResponse {
                        minter: contract_addr.to_string(),
                        cap: None,
                    }),
                    marketing: None,
                    total_supply: None,
                },
                &coins(1000, "atom"),
                "main contract",
                Option::from(ADDR1.to_string()),
            )
            .unwrap();

        let execute_mint_msg = ExecuteMsg::Mint(MintMsg {
            token_id: "1".to_string(),
            owner: contract_addr.to_string(),
            token_uri: None,
            extension: Option::from(Metadata {
                unit_denom: "mars".to_string(),
                price: 100,
                name: Option::from("Spaceship".to_string()),
                luggage: vec![Luggage {
                    denom: "oil".to_string(),
                    amount: Uint128::new(10),
                }],
            }),
        });

        let add_luggage_contract_msg: ExecuteMsg<Extension> = AddLuggageContract {
            address: oil_cw20_contract_addr.to_string(),
            denom: "oil".to_string(),
            code_id: 3,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &add_luggage_contract_msg,
            &[],
        )
        .unwrap();

        let _increase_allowance_msg = IncreaseAllowance {
            spender: contract_addr.to_string(),
            amount: Uint128::new(100),
            expires: None,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &execute_mint_msg,
            &[],
        )
        .unwrap();

        let query_money_contract_addr = QueryMsg::MoneyContract {};
        let money_contract_addr: MoneyContractResponse = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &query_money_contract_addr)
            .unwrap();

        assert_eq!(
            money_contract_addr.address.to_string(),
            "contract1".to_string()
        );

        app.execute_contract(
            Addr::unchecked(ADDR1),
            money_contract_addr.address,
            &IncreaseAllowance {
                spender: contract_addr.to_string(),
                amount: Uint128::new(100),
                expires: None,
            },
            &[],
        )
        .unwrap();

        let buy_nft_msg: ExecuteMsg<Extension> = ExecuteMsg::BuyNft {
            original_owner: contract_addr.to_string(),
            nft_id: "1".to_string(),
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &buy_nft_msg,
            &[],
        )
        .unwrap();

        let query_nft_msg = cw721::Cw721QueryMsg::OwnerOf {
            token_id: "1".to_string(),
            include_expired: Option::from(false),
        };

        let owner_of_1_res: cw721::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart("contract2".to_string(), &query_nft_msg)
            .unwrap();

        assert_eq!(owner_of_1_res.owner, ADDR1.to_string());

        let increase_allowance_msg = IncreaseAllowance {
            spender: contract_addr.to_string(),
            amount: Uint128::new(1000),
            expires: None,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            oil_cw20_contract_addr,
            &increase_allowance_msg,
            &[],
        )
        .unwrap();

        let load_luggage_msg: ExecuteMsg<Extension> = LoadLuggage {
            token_id: "1".to_string(),
            denom: "oil".to_string(),
            amount: 1000,
        };

        let _load_luggage_res = app
            .execute_contract(
                Addr::unchecked(ADDR1),
                contract_addr,
                &load_luggage_msg,
                &[],
            )
            .unwrap();

        let query_nft_info_msg = NftInfo {
            token_id: "1".to_string(),
        };
        let query_nft_info_res: NftInfoResponse<Extension> = app
            .wrap()
            .query_wasm_smart("contract2".to_string(), &query_nft_info_msg)
            .unwrap();

        assert_eq!(
            query_nft_info_res.extension.unwrap(),
            Metadata {
                unit_denom: "mars".to_string(),
                price: 100,
                name: Option::from("Spaceship".to_string()),
                luggage: vec![Luggage {
                    denom: "oil".to_string(),
                    amount: Uint128::new(1010),
                }],
            }
        );

        let query_balance_msg = cosmonaut_cw20::msg::QueryMsg::Balance {
            address: ADDR1.to_string(),
        };

        let query_balance_res: BalanceResponse = app
            .wrap()
            .query_wasm_smart("contract3".to_string(), &query_balance_msg)
            .unwrap();

        assert_eq!(query_balance_res.balance.to_string(), "9000")
    }
}
