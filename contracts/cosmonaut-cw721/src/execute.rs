use crate::state::{Extension, Luggage, MARSContract};
use crate::ContractError;
use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw721::Cw721ReceiveMsg;
use cw721_base::state::{Approval, TokenInfo};
use cw721_base::MintMsg;
use cw_utils::Expiration;

pub fn execute_transfer_nft(
    deps: DepsMut,
    env: Env,
    token_id: String,
    sender: Addr,
    recipient: String,
) -> Result<Response, ContractError> {
    _transfer(deps, &env, &token_id, &sender, &recipient)?;
    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("token_id", token_id)
        .add_attribute("from", sender)
        .add_attribute("to", recipient))
}

pub fn execute_send_nft(
    deps: DepsMut,
    env: Env,
    token_id: &String,
    sender: Addr,
    contract_addr: String,
    msg: Binary,
) -> Result<Response, ContractError> {
    _transfer(deps, &env, token_id, &sender, &contract_addr)?;

    let contract_msg = Cw721ReceiveMsg {
        sender: sender.to_string(),
        token_id: token_id.to_string(),
        msg,
    }
    .into_cosmos_msg(&contract_addr)?;

    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("token_id", token_id)
        .add_attribute("from", sender)
        .add_attribute("to", contract_addr)
        .add_message(contract_msg))
}

fn _transfer(
    deps: DepsMut,
    env: &Env,
    token_id: &str,
    sender: &Addr,
    recipient: &str,
) -> Result<(), ContractError> {
    let mars_contract = MARSContract::default();
    let token = mars_contract.tokens.may_load(deps.storage, token_id)?;

    if token.is_none() {
        return Err(ContractError::NotFound {});
    }

    check_can_send(&mars_contract, deps.as_ref(), env, sender, &token.unwrap())?;
    let recipient_addr = deps.api.addr_validate(recipient)?;
    mars_contract.tokens.update(
        deps.storage,
        token_id,
        |old_token_info: Option<TokenInfo<Extension>>| -> StdResult<_> {
            let mut new_token_info = old_token_info.unwrap();
            new_token_info.owner = recipient_addr;
            new_token_info.approvals = vec![];
            Ok(new_token_info)
        },
    )?;
    Ok(())
}

pub fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mint_msg: MintMsg<Extension>,
) -> Result<Response, ContractError> {
    let mars_contract = MARSContract::default();
    let response = mars_contract.mint(deps, env, info, mint_msg).unwrap();
    Ok(response)
}

fn check_can_send(
    contract: &MARSContract,
    deps: Deps,
    env: &Env,
    sender: &Addr,
    token: &TokenInfo<Extension>,
) -> Result<(), ContractError> {
    if token.owner == sender.as_ref() {
        return Ok(());
    }

    if token
        .approvals
        .iter()
        .any(|approval| approval.spender == sender.as_ref() && !approval.is_expired(&env.block))
    {
        return Ok(());
    }

    let operators = contract
        .operators
        .may_load(deps.storage, (&token.owner, sender))?;

    match operators {
        Some(expiration) => {
            if expiration.is_expired(&env.block) {
                Err(ContractError::Unauthorized {})
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::Unauthorized {}),
    }
}

pub fn execute_approve(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    token_id: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

    Ok(Response::new()
        .add_attribute("action", "approve")
        .add_attribute("sender", info.sender)
        .add_attribute("spender", spender)
        .add_attribute("token_id", token_id))
}

pub fn execute_approve_all(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    operator: String,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    let contract = MARSContract::default();

    let expires = expires.unwrap_or_default();
    if expires.is_expired(&env.block) {
        return Err(ContractError::Expired {});
    }

    let operator_addr = deps.api.addr_validate(&operator)?;
    contract
        .operators
        .save(deps.storage, (&info.sender, &operator_addr), &expires)?;

    Ok(Response::new()
        .add_attribute("action", "approve_all")
        .add_attribute("sender", info.sender)
        .add_attribute("operator", operator))
}

pub fn execute_revoke(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    spender: String,
    token_id: String,
) -> Result<Response, ContractError> {
    update_approvals(deps, &env, &info, &spender, &token_id, false, None)?;

    Ok(Response::new()
        .add_attribute("action", "revoke")
        .add_attribute("sender", info.sender)
        .add_attribute("spender", spender)
        .add_attribute("token_id", token_id))
}

pub fn execute_revoke_all(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    operator: String,
) -> Result<Response, ContractError> {
    let contract = MARSContract::default();
    let operator_addr = deps.api.addr_validate(&operator)?;
    contract
        .operators
        .remove(deps.storage, (&info.sender, &operator_addr));
    Ok(Response::new()
        .add_attribute("action", "revoke_all")
        .add_attribute("sender", info.sender)
        .add_attribute("operator", operator))
}

fn update_approvals(
    deps: DepsMut,
    env: &Env,
    info: &MessageInfo,
    spender: &str,
    token_id: &str,
    add: bool,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    let mars_contract = MARSContract::default();
    let mut token = mars_contract.tokens.load(deps.storage, token_id)?;

    check_can_approve(deps.as_ref(), env, info, &token, &mars_contract)?;
    let spender_addr = deps.api.addr_validate(spender)?;

    token.approvals = token
        .approvals
        .into_iter()
        .filter(|approval| approval.spender != spender_addr)
        .collect();
    if add {
        let expires = expires.unwrap_or_default();
        if expires.is_expired(&env.block) {
            return Err(ContractError::Expired {});
        }
        let approval = Approval {
            spender: spender_addr,
            expires,
        };
        token.approvals.push(approval);
    }
    mars_contract.tokens.save(deps.storage, token_id, &token)?;
    Ok(Response::new())
}

fn check_can_approve(
    deps: Deps,
    env: &Env,
    info: &MessageInfo,
    token: &TokenInfo<Extension>,
    contract: &MARSContract,
) -> Result<(), ContractError> {
    if token.owner == info.sender {
        return Ok(());
    }

    let operator = contract
        .operators
        .may_load(deps.storage, (&token.owner, &info.sender))?;
    match operator {
        Some(expiration) => {
            if expiration.is_expired(&env.block) {
                Err(ContractError::Unauthorized {})
            } else {
                Ok(())
            }
        }
        None => Err(ContractError::Unauthorized {}),
    }
}

pub fn execute_burn(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: String,
) -> Result<Response, ContractError> {
    let contract = MARSContract::default();
    let token = contract.tokens.load(deps.storage, &token_id)?;

    check_can_send(&contract, deps.as_ref(), &env, &info.sender, &token)?;

    contract.tokens.remove(deps.storage, &token_id)?;
    contract.decrement_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "burn")
        .add_attribute("sender", info.sender)
        .add_attribute("token_id", token_id))
}

pub fn execute_set_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter: String,
) -> Result<Response, ContractError> {
    let minter_addr = deps.api.addr_validate(&minter)?;
    let mars_contract = MARSContract::default();

    if mars_contract.minter(deps.as_ref())?.minter == info.sender {
        mars_contract.minter.save(deps.storage, &minter_addr)?;
    } else {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("minter", minter))
}

pub fn execute_load_luggage(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let contract = MARSContract::default();
    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension.unwrap();

    let candidate_idx = extension.luggage.iter().position(|l| l.denom == denom);

    if let Some(idx) = candidate_idx {
        extension.luggage[idx].amount = extension.luggage[candidate_idx.unwrap()]
            .amount
            .checked_add(Uint128::new(amount))
            .unwrap();
    } else {
        extension.luggage.push(Luggage {
            denom: denom.clone(),
            amount: Uint128::new(amount),
        })
    }

    token.extension = Extension::from(extension);
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "load_luggage")
        .add_attribute("token_id", token_id)
        .add_attribute("luggage", denom)
        .add_attribute("amount", amount.to_string()))
}