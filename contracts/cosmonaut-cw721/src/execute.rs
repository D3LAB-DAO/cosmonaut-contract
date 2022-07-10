use std::convert::{TryFrom, TryInto};
use crate::msg::ExecuteMsg;
use crate::state::{Extension, Freight};
use crate::ContractError;
use cosmwasm_std::{Addr, Deps, DepsMut, Empty, Env, MessageInfo, Response, Uint128};
use cw721_base::state::TokenInfo;
use cw721_base::Cw721Contract;

pub trait BaseExecute {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError>;
}

impl<'a> BaseExecute for Cw721Contract<'a, Extension, Empty> {
    fn base_execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        let cw721_msg = msg.try_into()?;
        let execute_res = self.execute(deps, env, info, cw721_msg);
        match execute_res {
            Ok(res) => Ok(res),
            Err(err) => Err(ContractError::try_from(err)?),
        }
    }
}

fn check_can_send(
    contract: &Cw721Contract<Extension, Empty>,
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

pub fn execute_set_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter: String,
) -> Result<Response, ContractError> {
    let minter_addr = deps.api.addr_validate(&minter)?;
    let cosmonaut_contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();

    if cosmonaut_contract.minter(deps.as_ref())?.minter == info.sender {
        cosmonaut_contract.minter.save(deps.storage, &minter_addr)?;
    } else {
        return Err(ContractError::Unauthorized {});
    }

    Ok(Response::new()
        .add_attribute("action", "set_minter")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("minter", minter))
}

pub fn execute_load_freight(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: Uint128,
    unit_weight: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension;

    // iterate freight to find target cw20-tokens by denom
    let candidate_idx = extension.freight.iter().position(|l| l.denom == denom);
    // if there is token with given denom
    if let Some(idx) = candidate_idx {
        // update token amount
        extension.freight[idx].amount = extension.freight[idx]
            .amount
            .checked_add(amount)
            .unwrap();
    } else {
        // if not, push a new freight data
        extension.freight.push(Freight {
            denom: denom.clone(),
            amount,
            unit_weight: unit_weight.u128(),
        })
    }

    token.extension = Extension::from(extension);
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "load_freight")
        .add_attribute("token_id", token_id)
        .add_attribute("freight", denom)
        .add_attribute("amount", amount.to_string())
        .add_attribute("unit_weight", unit_weight.to_string()))
}

pub fn execute_unload_freight(
    deps: DepsMut,
    token_id: String,
    denom: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    let mut token = contract.tokens.load(deps.storage, &token_id)?;
    let mut extension = token.extension;

    let candidate_idx = extension.freight.iter().position(|l| l.denom == denom);
    if let Some(idx) = candidate_idx {
        if extension.freight[idx].amount.u128() - amount.u128() == 0 {
            extension.freight.remove(idx);
        } else {
            extension.freight[idx].amount = extension.freight[idx]
                .amount
                .checked_sub(amount)
                .unwrap();
        }
    } else {
        return Err(ContractError::NotFound {});
    }
    token.extension = Extension::from(extension);
    contract.tokens.save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "unload_lugagge")
        .add_attribute("token_id", token_id)
        .add_attribute("freight", denom)
        .add_attribute("amount", amount.to_string()))
}

pub fn execute_decrease_health(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    token_id: String,
    value: Uint128,
) -> Result<Response, ContractError> {
    let cosmonaut_contract: Cw721Contract<Extension, Empty> = Cw721Contract::default();
    let mut token = cosmonaut_contract.tokens.load(deps.storage, &token_id)?;
    check_can_send(
        &cosmonaut_contract,
        deps.as_ref(),
        &env,
        &info.sender,
        &token,
    )?;
    let mut extension = token.extension;

    // handle with negative overflow
    extension.health = extension.health.saturating_sub(value.u128());
    token.extension = Extension::from(extension);
    cosmonaut_contract
        .tokens
        .save(deps.storage, &token_id, &token)?;

    Ok(Response::new()
        .add_attribute("action", "decrease_health")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("token_id", token_id.to_string())
        .add_attribute("value", value.to_string()))
}
