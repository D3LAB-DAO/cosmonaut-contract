#[cfg(test)]
mod tests {
    use crate::contract;
    use crate::msg::ExecuteMsg::{
        AddFreightContract, BuyFreightToken, BuyMoneyToken, LoadFreight, UnLoadFreight,
    };
    use crate::msg::{
        ContractInitInfo, ExecuteMsg, InstantiateMsg, MoneyContractResponse, QueryMsg,
    };
    use cosmonaut_cw20::msg::ExecuteMsg::IncreaseAllowance;
    use cosmonaut_cw20::msg::{BalanceResponse, MinterResponse};
    use cosmonaut_cw721::state::{Extension, Freight, Metadata};
    use cosmwasm_std::{coin, Addr, BlockInfo, Coin, Empty, Uint128};
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

    pub fn next_block(block: &mut BlockInfo) {
        block.time = block.time.plus_seconds(5);
        block.height += 1;
    }

    #[test]
    fn test_execute() {
        let init_funds = vec![coin(5000, "uatom")];
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
            money_cw20_id: cw20_code_id,
            spaceship_cw721_id: cw721_code_id,
        };

        let contract_addr = app
            .instantiate_contract(
                main_contract_id,
                Addr::unchecked(ADDR1),
                &instantiate_msg,
                &[],
                "main contract",
                Some(ADDR1.to_string()),
            )
            .unwrap();

        let buy_money_token_msg: ExecuteMsg<Extension> = BuyMoneyToken { amount: 2000 };
        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &buy_money_token_msg,
            &[coin(2000, "uatom")],
        )
        .unwrap();

        let query_balance_of_addr = app
            .wrap()
            .query_balance(Addr::unchecked(ADDR1), "uatom")
            .unwrap();

        // Init : 5000
        // Buy money token 2000
        // 3000 left
        assert_eq!(
            query_balance_of_addr,
            Coin {
                amount: Uint128::new(3000),
                denom: "uatom".to_string(),
            }
        );

        let oil_cw20_contract_addr = app
            .instantiate_contract(
                cw20_code_id,
                Addr::unchecked(ADDR1),
                &cosmonaut_cw20::msg::InstantiateMsg {
                    name: "OIL".to_string(),
                    symbol: "oil".to_string(),
                    decimals: 6,
                    initial_balances: vec![],
                    mint: Some(MinterResponse {
                        minter: contract_addr.to_string(),
                        cap: None,
                    }),
                    marketing: None,
                    total_supply: None,
                },
                &[],
                "main contract",
                Some(ADDR1.to_string()),
            )
            .unwrap();

        let add_freight_contract_msg: ExecuteMsg<Extension> = AddFreightContract {
            address: oil_cw20_contract_addr.to_string(),
            denom: "oil".to_string(),
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &add_freight_contract_msg,
            &[],
        )
        .unwrap();

        let execute_mint_msg = ExecuteMsg::Mint(MintMsg {
            token_id: "1".to_string(),
            owner: contract_addr.to_string(),
            token_uri: None,
            extension: Some(Metadata {
                unit_denom: "mars".to_string(),
                price: 500,
                name: Some("Spaceship".to_string()),
                freight: vec![],
                health: 30,
            }),
        });

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

        app.execute_contract(
            Addr::unchecked(ADDR1),
            money_contract_addr.clone().address,
            &IncreaseAllowance {
                spender: contract_addr.to_string(),
                amount: Uint128::new(500),
                expires: None,
            },
            &[],
        )
        .unwrap();

        let increase_money_allowance_msg = IncreaseAllowance {
            spender: contract_addr.to_string(),
            amount: Uint128::new(1000),
            expires: None,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            money_contract_addr.clone().address,
            &increase_money_allowance_msg,
            &[],
        )
        .unwrap();

        let buy_freight_token_msg: ExecuteMsg<Extension> = BuyFreightToken {
            denom: "oil".to_string(),
            amount: 1000,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &buy_freight_token_msg,
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
            include_expired: Some(false),
        };

        let owner_of_1_res: cw721::OwnerOfResponse = app
            .wrap()
            .query_wasm_smart("contract2".to_string(), &query_nft_msg)
            .unwrap();

        //ADDR1 bought nft, so he is the owner
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

        let load_freight_msg: ExecuteMsg<Extension> = LoadFreight {
            token_id: "1".to_string(),
            denom: "oil".to_string(),
            amount: 1000,
            unit_weight: 840,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &load_freight_msg,
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

        // Loaded 1000 oil to NFT
        assert_eq!(
            query_nft_info_res.extension.unwrap(),
            Metadata {
                unit_denom: "mars".to_string(),
                price: 500,
                name: Some("Spaceship".to_string()),
                freight: vec![Freight {
                    denom: "oil".to_string(),
                    amount: Uint128::new(1000),
                    unit_weight: 840,
                }],
                health: 30,
            }
        );

        let query_balance_msg = cosmonaut_cw20::msg::QueryMsg::Balance {
            address: ADDR1.to_string(),
        };

        let query_balance_res: BalanceResponse = app
            .wrap()
            .query_wasm_smart("contract3".to_string(), &query_balance_msg)
            .unwrap();

        // ADDR1 loaded 1000 oil to nft, so balacne is 0
        assert_eq!(query_balance_res.balance.to_string(), "0");

        let unload_freight_msg: ExecuteMsg<Extension> = UnLoadFreight {
            token_id: "1".to_string(),
            denom: "oil".to_string(),
            amount: 100,
        };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr.clone(),
            &unload_freight_msg,
            &[],
        )
        .unwrap();

        let query_balance_res: BalanceResponse = app
            .wrap()
            .query_wasm_smart("contract3".to_string(), &query_balance_msg)
            .unwrap();

        // ADDR1 unloaded 100 oil, so balance is 100
        assert_eq!(query_balance_res.balance.to_string(), "100");

        let query_balance_of_main_contract = app
            .wrap()
            .query_balance(Addr::unchecked(contract_addr.clone()), "uatom")
            .unwrap();
        // ADDR1 bought 2000 cw20-tokens money token with 2000 atom, main contract's atom balance is 2000
        assert_eq!(
            query_balance_of_main_contract,
            Coin {
                amount: Uint128::new(2000),
                denom: "uatom".to_string(),
            }
        );

        let query_cw20_balance_res: BalanceResponse = app
            .wrap()
            .query_wasm_smart(
                money_contract_addr.address.to_string(),
                &cosmonaut_cw20::msg::QueryMsg::Balance {
                    address: ADDR1.to_string(),
                },
            )
            .unwrap();

        // ADDR1 bought a nft which is 500 money token, balance is 500
        assert_eq!(query_cw20_balance_res.balance, Uint128::new(500));

        let approve_nft_msg: cosmonaut_cw721::msg::ExecuteMsg =
            cosmonaut_cw721::msg::ExecuteMsg::Approve {
                spender: contract_addr.to_string(),
                token_id: "1".to_string(),
                expires: None,
            };

        app.execute_contract(
            Addr::unchecked(ADDR1),
            Addr::unchecked("contract2".to_string()),
            &approve_nft_msg,
            &[],
        )
        .unwrap();

        let play_game_msg: ExecuteMsg = ExecuteMsg::PlayGame {
            token_id: "1".to_string(),
            epoch: Uint128::new(5),
        };

        app.execute_contract(Addr::unchecked(ADDR1), contract_addr, &play_game_msg, &[])
            .unwrap();

        let query_nft_info_msg = NftInfo {
            token_id: "1".to_string(),
        };
        let query_nft_info_res: NftInfoResponse<Extension> = app
            .wrap()
            .query_wasm_smart("contract2".to_string(), &query_nft_info_msg)
            .unwrap();

        assert_eq!(query_nft_info_res.extension.unwrap().health, 0);
        app.update_block(next_block);
    }
}
