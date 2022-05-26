#[cfg(test)]
mod tests {
    use crate::contract;
    use crate::msg::{ContractInitInfo, ExecuteMsg, InstantiateMsg};
    use crate::state::{Extension, Metadata};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw721_base::MintMsg;
    use cw_multi_test::{custom_app, Executor};

    const ADDR1: &str = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";

    #[test]
    fn test_execute() {
        let init_funds = vec![coin(100000, "atom")];
        let mut app = custom_app::<ExecuteMsg<Extension>, Empty, _>(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADDR1), init_funds)
                .unwrap();
        });

        let cw20_code_id = app.store_code(cosmonaut_cw20::contract::contract());
        let cw721_code_id = app.store_code(cosmonaut_cw721::contract::contract());
        let main_contract_id = app.store_code(contract::contract());

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

        let execute_mint_msg = ExecuteMsg::Mint(MintMsg {
            token_id: "1".to_string(),
            owner: contract_addr.to_string(),
            token_uri: None,
            extension: Option::from(Metadata {
                unit_denom: "mars".to_string(),
                price: 100,
                image: None,
                image_data: None,
                external_url: None,
                description: None,
                name: None,
                attributes: None,
                background_color: None,
                animation_url: None,
                youtube_url: None,
            }),
        });

        app.execute_contract(
            Addr::unchecked(ADDR1),
            contract_addr,
            &execute_mint_msg,
            &[],
        )
        .unwrap();
        // let msg = cosmonaut_cw20::msg::InstantiateMsg {
        //     name: "MARS".to_string(),
        //     symbol: "mars".to_string(),
        //     decimals: 6,
        //     initial_balances: vec![Cw20Coin { address: ADDR1.to_string(), amount: Uint128::new(100000) }],
        //     mint: Option::from(cosmonaut_cw20::msg::MinterResponse { minter: ADDR1.to_string(), cap: None }),
        //     marketing: None,
        // };
        //
        // let contract_addr = app.instantiate_contract(
        //     cw20_code_id,
        //     Addr::unchecked(ADDR1.clone()),
        //     &msg,
        //     &coins(50, "atom"),
        //     "Cw20",
        //     Option::from(ADDR1.to_string()),
        // ).unwrap();
        //
        // let contract_data = app.contract_data(&contract_addr).unwrap();
        //
        // assert_eq!(contract_data.code_id, 1);
        // assert_eq!(contract_addr, Addr::unchecked("contract0"));
        //
        // let res = app.execute_contract(
        //     Addr::unchecked(ADDR1),
        //     contract_addr.clone(),
        //     &cosmonaut_cw20::msg::ExecuteMsg::Transfer { recipient: ADDR2.to_string(), amount: Uint128::new(10) }, &[])
        //     .unwrap();
        //
        // let transfer = &res.events[1];
        //
        // assert_eq!(
        //     transfer.attributes,
        //     [
        //         Attribute { key: "_contract_addr".to_string(), value: "contract0".to_string() },
        //         Attribute { key: "action".to_string(), value: "transfer".to_string() },
        //         Attribute { key: "sender".to_string(), value: ADDR1.to_string() },
        //         Attribute { key: "recipient".to_string(), value: ADDR2.to_string() },
        //         Attribute { key: "amount".to_string(), value: "10".to_string() }
        //     ]
        // );
        //
        // let query_msg = cosmonaut_cw20::msg::QueryMsg::Balance {
        //     address: ADDR2.to_string()
        // };
        //
        // let query_res: cosmonaut_cw20::msg::QueryResponse = app
        //     .wrap()
        //     .query_wasm_smart(&contract_addr, &query_msg)
        //     .unwrap();
        //
        // assert_eq!(
        //     &query_res,
        //     &cosmonaut_cw20::msg::QueryResponse::BalanceResponse {
        //         balance: Uint128::new(10)
        //     }
        // );
    }
}
