#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::ExecuteMsg;
    use crate::state::{CosmonautContract, Extension, Metadata};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{attr, from_binary, DepsMut, Response};
    use cw721::{Approval, ApprovalsResponse, NftInfoResponse, NumTokensResponse, OwnerOfResponse};
    use cw721_base::{InstantiateMsg, MintMsg, QueryMsg};
    use cw_utils::Expiration;
    use serde::de::Unexpected::Option;
    use std::collections::HashMap;

    const MINTER: &str = "juno18zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const STRANGER: &str = "juno17zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";
    const STRANGER2: &str = "juno16zfp9u7zxg3gel4r3txa2jqxme7jkw7d972flm";

    fn setup_contract(deps: DepsMut) -> CosmonautContract {
        let contract = CosmonautContract::default();
        let msg = InstantiateMsg {
            name: "mars".to_string(),
            symbol: "MARS".to_string(),
            minter: MINTER.to_string(),
        };
        let info = mock_info(MINTER, &[]);
        let res = instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(
            res.attributes,
            [attr("action", "instantiate"), attr("sender", MINTER)]
        );
        contract
    }

    fn mint_a_nft(deps: DepsMut, sender: &str, owner: &str) -> Response {
        let metadata = Metadata {
            unit_denom: "mars".to_string(),
            price: 1000,
            name: None,
            freight: vec![],
            health: 0,
        };
        let mint_msg = MintMsg {
            token_id: "1".to_string(),
            owner: owner.to_string(),
            token_uri: Option::from(
                "https://docs.cosmwasm.com/cw-plus/0.9.0/cw721/spec".to_string(),
            ),
            extension: Option::from(metadata),
        };

        let execute_mint_msg = ExecuteMsg::Mint(mint_msg);
        execute(deps, mock_env(), mock_info(sender, &[]), execute_mint_msg).unwrap()
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
    }

    #[test]
    fn test_execute_mint() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let res = mint_a_nft(deps.as_mut(), MINTER, STRANGER);
        assert_eq!(
            res.attributes,
            [
                attr("action", "mint"),
                attr("minter", MINTER),
                attr("owner", STRANGER),
                attr("token_id", "1")
            ]
        );
    }

    #[test]
    fn test_transfer() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        mint_a_nft(deps.as_mut(), MINTER, MINTER);

        let transfer_msg: ExecuteMsg<Extension> = ExecuteMsg::TransferNft {
            recipient: STRANGER.to_string(),
            token_id: "1".to_string(),
        };
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            transfer_msg,
        )
        .unwrap();
        assert_eq!(
            res.attributes,
            [
                attr("action", "transfer"),
                attr("token_id", "1"),
                attr("from", MINTER),
                attr("to", STRANGER)
            ]
        );
    }

    #[test]
    fn test_approve() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        mint_a_nft(deps.as_mut(), MINTER, MINTER);
        let approve_msg: ExecuteMsg<Extension> = ExecuteMsg::Approve {
            spender: STRANGER.to_string(),
            token_id: "1".to_string(),
            expires: Option::from(Expiration::AtHeight(20000)),
        };
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            approve_msg,
        )
        .unwrap();

        let transfer_msg: ExecuteMsg<Extension> = ExecuteMsg::TransferNft {
            recipient: STRANGER.to_string(),
            token_id: "1".to_string(),
        };
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(STRANGER, &[]),
            transfer_msg,
        )
        .unwrap();
        assert_eq!(
            res.attributes,
            [
                attr("action", "transfer"),
                attr("token_id", "1"),
                attr("from", STRANGER),
                attr("to", STRANGER)
            ]
        );
    }

    #[test]
    fn test_revoke() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        mint_a_nft(deps.as_mut(), MINTER, MINTER);
        let approve_msg: ExecuteMsg<Extension> = ExecuteMsg::Approve {
            spender: STRANGER.to_string(),
            token_id: "1".to_string(),
            expires: Option::from(Expiration::AtHeight(20000)),
        };
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            approve_msg,
        )
        .unwrap();

        let revoke_msg: ExecuteMsg<Extension> = ExecuteMsg::Revoke {
            spender: STRANGER.to_string(),
            token_id: "1".to_string(),
        };

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            revoke_msg,
        )
        .unwrap();

        let transfer_msg: ExecuteMsg<Extension> = ExecuteMsg::TransferNft {
            recipient: STRANGER.to_string(),
            token_id: "1".to_string(),
        };
        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(STRANGER, &[]),
            transfer_msg,
        )
        .unwrap_err();
    }

    #[test]
    fn execute_approve_revoke_all() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        mint_a_nft(deps.as_mut(), MINTER, MINTER);

        let approve_all_msg = ExecuteMsg::ApproveAll {
            operator: STRANGER.to_string(),
            expires: Option::from(Expiration::AtHeight(20000)),
        };

        let approve_all_res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            approve_all_msg,
        )
        .unwrap();

        assert_eq!(
            approve_all_res.attributes,
            [
                attr("action", "approve_all"),
                attr("sender", MINTER),
                attr("operator", STRANGER)
            ]
        );

        let transfer_msg: ExecuteMsg<Extension> = ExecuteMsg::TransferNft {
            recipient: STRANGER.to_string(),
            token_id: "1".to_string(),
        };

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(STRANGER, &[]),
            transfer_msg,
        )
        .unwrap();

        let _revoke_all_msg = ExecuteMsg::RevokeAll {
            operator: STRANGER.to_string(),
        };

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            _revoke_all_msg,
        )
        .unwrap();
    }

    #[test]
    fn test_query() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        mint_a_nft(deps.as_mut(), MINTER, MINTER);

        let approve_msg = ExecuteMsg::Approve {
            spender: STRANGER.to_string(),
            token_id: "1".to_string(),
            expires: Option::from(Expiration::AtHeight(20000)),
        };

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            approve_msg,
        )
        .unwrap();

        let approve_msg2 = ExecuteMsg::Approve {
            spender: STRANGER2.to_string(),
            token_id: "1".to_string(),
            expires: Option::from(Expiration::AtHeight(18000)),
        };

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(MINTER, &[]),
            approve_msg2,
        )
        .unwrap();

        let query_owner_msg = QueryMsg::OwnerOf {
            token_id: "1".to_string(),
            include_expired: Option::from(true),
        };

        let query_res_bin = query(deps.as_ref(), mock_env(), query_owner_msg).unwrap();

        let query_res: OwnerOfResponse = from_binary(&query_res_bin).unwrap();
        assert_eq!(
            query_res,
            OwnerOfResponse {
                owner: MINTER.to_string(),
                approvals: vec![
                    Approval {
                        spender: STRANGER.to_string(),
                        expires: Expiration::AtHeight(20000),
                    },
                    Approval {
                        spender: STRANGER2.to_string(),
                        expires: Expiration::AtHeight(18000),
                    },
                ],
            }
        );

        let query_approvals_msg = QueryMsg::Approvals {
            token_id: "1".to_string(),
            include_expired: Option::from(true),
        };

        let query_approvals_res_bin =
            query(deps.as_ref(), mock_env(), query_approvals_msg).unwrap();

        let query_approvals_res: ApprovalsResponse = from_binary(&query_approvals_res_bin).unwrap();

        assert_eq!(
            query_approvals_res,
            ApprovalsResponse {
                approvals: vec![
                    Approval {
                        spender: STRANGER.to_string(),
                        expires: Expiration::AtHeight(20000),
                    },
                    Approval {
                        spender: STRANGER2.to_string(),
                        expires: Expiration::AtHeight(18000),
                    },
                ]
            }
        );

        let query_token_num_msg = QueryMsg::NumTokens {};

        let query_token_num_res_bin =
            query(deps.as_ref(), mock_env(), query_token_num_msg).unwrap();

        let query_token_num_res: NumTokensResponse = from_binary(&query_token_num_res_bin).unwrap();
        assert_eq!(query_token_num_res, NumTokensResponse { count: 1 });

        let query_nft_info_msg = QueryMsg::NftInfo {
            token_id: "1".to_string(),
        };

        let query_nft_info_bin = query(deps.as_ref(), mock_env(), query_nft_info_msg).unwrap();

        let query_nft_info_res: NftInfoResponse<Extension> =
            from_binary(&query_nft_info_bin).unwrap();
        assert_eq!(
            query_nft_info_res,
            NftInfoResponse {
                token_uri: Some("https://docs.cosmwasm.com/cw-plus/0.9.0/cw721/spec".to_string()),

                extension: Option::from(Metadata {
                    unit_denom: "mars".to_string(),
                    price: 1000,
                    name: None,
                    freight: vec![],
                    health: 10
                }),
            }
        )
    }
}
