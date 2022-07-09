use crate::error::ContractError;
use crate::msg::AllowanceResponse;
use crate::state::{ALLOWANCES, BALANCES, TOKEN_INFO};
use cosmwasm_std::{
    Addr, Binary, BlockInfo, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage,
    Uint128,
};
use cw20::Cw20ReceiveMsg;
use cw_utils::Expiration;

pub fn transfer(
    deps: DepsMut,
    sender: Addr,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let sender_balance = BALANCES.load(deps.storage, &sender)?;
    if sender_balance < amount {
        return Err(ContractError::NotEnoughBalance {
            current_balance: sender_balance.u128(),
            amount: amount.u128(),
        });
    }

    BALANCES.update(deps.storage, &sender, |before_amount| -> StdResult<_> {
        Ok(before_amount.unwrap_or_default().checked_sub(amount)?)
    })?;

    BALANCES.update(
        deps.storage,
        &Addr::unchecked(&recipient),
        |before_amount| -> StdResult<_> {
            Ok(before_amount.unwrap_or_default().checked_add(amount)?)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("sender", sender.to_string())
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string()))
}

pub fn send(
    deps: DepsMut,
    sender: Addr,
    contract: String,
    amount: Uint128,
    msg: Binary,
) -> Result<Response, ContractError> {
    if sender == contract {
        return Err(ContractError::SameAddress {});
    }

    let sender_balance = BALANCES.load(deps.storage, &sender)?;
    if sender_balance < amount {
        return Err(ContractError::NotEnoughBalance {
            current_balance: sender_balance.u128(),
            amount: amount.u128(),
        });
    }

    let contract = deps.api.addr_validate(&contract)?;

    BALANCES.update(deps.storage, &sender, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })?;

    BALANCES.update(deps.storage, &contract, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_add(amount)?)
    })?;

    let contract_msg = Cw20ReceiveMsg {
        sender: sender.to_string(),
        amount,
        msg,
    }
    .into_cosmos_msg(&contract)?;

    Ok(Response::new()
        .add_attribute("action", "send")
        .add_attribute("from", sender.to_string())
        .add_attribute("to", contract.to_string())
        .add_attribute("amount", amount.to_string())
        .add_message(contract_msg))
}

pub fn mint(
    deps: DepsMut,
    sender: Addr,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let token_info = TOKEN_INFO.load(deps.storage)?;
    let minter_data = token_info.mint.as_ref().unwrap();
    if minter_data.minter != sender {
        return Err(ContractError::Unauthorized {});
    }

    token_info
        .total_supply
        .unwrap_or_default()
        .checked_add(amount)
        .map_err(StdError::overflow)?;

    if let Some(limit) = token_info.get_cap() {
        if token_info.total_supply.unwrap_or_default() > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }
    TOKEN_INFO.save(deps.storage, &token_info)?;

    let validate_recipient = deps.api.addr_validate(&recipient)?;
    BALANCES.update(
        deps.storage,
        &validate_recipient,
        |balance| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
    )?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("recipient", recipient)
        .add_attribute("amount", amount.to_string()))
}

pub fn increase_allowance(
    deps: DepsMut,
    owner: Addr,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    if spender == owner {
        return Err(ContractError::SameAddress {});
    }

    let owner_balance = BALANCES.load(deps.storage, &Addr::unchecked(&owner))?;

    if owner_balance < amount {
        return Err(ContractError::NotEnoughBalance {
            current_balance: owner_balance.u128(),
            amount: amount.u128(),
        });
    }
    let spender_addr = deps.api.addr_validate(&spender)?;

    ALLOWANCES.update(deps.storage, (&owner, &spender_addr), |a| -> StdResult<_> {
        let mut allowance_res = a.unwrap_or_default();
        if let Some(exp) = expires {
            allowance_res.expires = exp;
        }
        allowance_res.allowance += amount;
        Ok(allowance_res)
    })?;

    Ok(Response::new()
        .add_attribute("action", "increase_allowance".to_string())
        .add_attribute("owner", owner.to_string())
        .add_attribute("spender", spender)
        .add_attribute("amount", amount.to_string()))
}

pub fn decrease_allowance(
    deps: DepsMut,
    owner: Addr,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
) -> Result<Response, ContractError> {
    if spender == owner {
        return Err(ContractError::SameAddress {});
    }

    let spender_addr = deps.api.addr_validate(&spender)?;
    let allowance_res = ALLOWANCES.load(deps.storage, (&owner, &spender_addr))?;

    if amount < allowance_res.allowance {
        ALLOWANCES.update(
            deps.storage,
            (&owner, &spender_addr),
            |allow| -> StdResult<_> {
                let mut new_allow = allow.unwrap_or_default();
                new_allow.allowance = new_allow.allowance.checked_sub(amount)?;
                if let Some(exp) = expires {
                    new_allow.expires = exp;
                }
                Ok(new_allow)
            },
        )?;
    } else {
        ALLOWANCES.remove(deps.storage, (&owner, &spender_addr));
    }

    Ok(Response::new()
        .add_attribute("action", "decrease_allowance".to_string())
        .add_attribute("owner", owner.to_string())
        .add_attribute("spender", spender)
        .add_attribute("amount", amount.to_string()))
}

pub fn transfer_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let recipient_addr = deps.api.addr_validate(&recipient)?;
    let owner_addr = deps.api.addr_validate(&owner)?;
    deduct_allowance(deps.storage, &owner_addr, &info.sender, &env.block, amount)?;

    BALANCES.update(deps.storage, &owner_addr, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })?;

    BALANCES.update(deps.storage, &recipient_addr, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_add(amount)?)
    })?;

    Ok(Response::new()
        .add_attribute("action", "transfer_from")
        .add_attribute("from", owner)
        .add_attribute("to", recipient_addr)
        .add_attribute("by", info.sender)
        .add_attribute("amount", amount))
}

fn deduct_allowance(
    storage: &mut dyn Storage,
    owner: &Addr,
    spender: &Addr,
    block: &BlockInfo,
    amount: Uint128,
) -> Result<AllowanceResponse, ContractError> {
    ALLOWANCES.update(storage, (owner, spender), |current| match current {
        Some(mut allowance) => {
            if allowance.expires.is_expired(block) {
                Err(ContractError::Expired {})
            } else {
                allowance.allowance = allowance
                    .allowance
                    .checked_sub(amount)
                    .map_err(StdError::overflow)?;
                Ok(allowance)
            }
        }
        None => Err(ContractError::Unauthorized {}),
    })
}

pub fn burn(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    if amount == Uint128::zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    BALANCES.update(deps.storage, &info.sender, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })?;

    TOKEN_INFO.update(deps.storage, |mut info| -> StdResult<_> {
        info.total_supply = Some(info.total_supply.unwrap_or_default().checked_sub(amount)?);
        Ok(info)
    })?;

    Ok(Response::new()
        .add_attribute("action", "burn")
        .add_attribute("from", info.sender)
        .add_attribute("amount", amount))
}

pub fn burn_from(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    owner: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    deduct_allowance(deps.storage, &owner_addr, &info.sender, &env.block, amount)?;

    BALANCES.update(deps.storage, &owner_addr, |balance| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })?;

    Ok(Response::new()
        .add_attribute("action", "burn")
        .add_attribute("owner", owner)
        .add_attribute("amount", amount.to_string()))
}
