use crate::state::Extension;
use cosmwasm_std::{Deps, Empty, Env, StdResult};
use cw721::{
    AllNftInfoResponse, ApprovalsResponse, ContractInfoResponse, Cw721Query, NftInfoResponse,
    NumTokensResponse, OwnerOfResponse, TokensResponse,
};
use cw721_base::{Cw721Contract, MinterResponse};

pub fn minter(deps: Deps) -> StdResult<MinterResponse> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.minter(deps)
}

pub fn owner_of(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> StdResult<OwnerOfResponse> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.owner_of(deps, env, token_id, include_expired)
}

pub fn approved_for_all(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> StdResult<ApprovalsResponse> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.approvals(deps, env, token_id, include_expired)
}

pub fn num_tokens(deps: Deps) -> StdResult<NumTokensResponse> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.num_tokens(deps)
}

pub fn nft_info(deps: Deps, token_id: String) -> StdResult<NftInfoResponse<Extension>> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.nft_info(deps, token_id)
}

pub fn all_nft_info(
    deps: Deps,
    env: Env,
    token_id: String,
    include_expired: bool,
) -> StdResult<AllNftInfoResponse<Extension>> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.all_nft_info(deps, env, token_id, include_expired)
}

pub fn tokens(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.tokens(deps, owner, start_after, limit)
}

pub fn contract_info(deps: Deps) -> StdResult<ContractInfoResponse> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    contract.contract_info(deps)
}
