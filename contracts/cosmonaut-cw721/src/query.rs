use crate::state::{CosmonautContract, Extension};
use cosmwasm_std::{Deps, Env, StdResult};
use cw721::{
    ApprovalsResponse, Cw721Query, NftInfoResponse, NumTokensResponse, OwnerOfResponse,
    TokensResponse,
};

pub fn query_owner_of(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> StdResult<OwnerOfResponse> {
    let contract = CosmonautContract::default();
    contract.owner_of(deps, env, token_id, include_expired)
}

pub fn query_approved_for_all(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> StdResult<ApprovalsResponse> {
    let contract = CosmonautContract::default();
    contract.approvals(deps, env, token_id, include_expired)
}

pub fn query_num_tokens(deps: Deps) -> StdResult<NumTokensResponse> {
    let contract = CosmonautContract::default();
    contract.num_tokens(deps)
}

pub fn query_nft_info(deps: Deps, token_id: String) -> StdResult<NftInfoResponse<Extension>> {
    let contract = CosmonautContract::default();
    contract.nft_info(deps, token_id)
}

pub fn query_tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let contract = CosmonautContract::default();
    contract.tokens(deps, owner, start_after, limit)
}
