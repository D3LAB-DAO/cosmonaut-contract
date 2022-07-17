* ### contract.rs

q1)
```rust
.add_submessages([
          instantiate_cw20_money_contract,
          instantiate_cw20_fuel_contract,
          instantiate_cw721_spaceship_contract,
      ])
```

q2)
```rust
let res = parse_reply_instantiate_data(msg).unwrap();
CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
    config.spaceship_cw721_contract = Addr::unchecked(res.contract_address);
    Ok(config)
})?;
```

* ### execute.rs

q3)
```rust
let token_balance: BalanceResponse = deps.querier.query_wasm_smart(
    config.money_cw20_contract.as_ref(),
    &cw20_base::msg::QueryMsg::Balance {
        address: info.sender.to_string(),
    },
)?;
```

q4)
```rust
let transfer_nft_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
    contract_addr: config.spaceship_cw721_contract.to_string(),
    msg: to_binary(&transfer_nft_msg)?,
    funds: vec![],
});
```

q5)
```rust
if amount * unit_weight > Uint128::new(MAX_FREIGHT_WEIGHT) {
    return Err(ContractError::FrightOverloaded {});
}
```

q6)
```rust
let target_contract_addr = config
    .freight_contracts
    .into_iter()
    .find(|c| c.denom == denom);

if target_contract_addr.is_none() {
    return Err(ContractError::TokenNotFound {});
}
```

q7)
```rust
.add_messages([burn_cw20_token_msg_wrap, load_freight_msg_wrap])
```

q8)
```rust
let mint_cw20_token_msg_wrap = CosmosMsg::Wasm(WasmMsg::Execute {
    contract_addr: target_contract_addr.unwrap().address,
    msg: to_binary(&cw20_base::msg::ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount,
    })?,
    funds: vec![],
});
```

q9)
```rust
let atom_income = income_asset
  .into_iter()
  .find(|coin| coin.denom == "uatom")
  .unwrap_or_else(|| coin(0, "uatom"));
```

q10)
```rust
let burn_money_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
    contract_addr: config.money_cw20_contract.to_string(),
    msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::BurnFrom {
        owner: info.sender.to_string(),
        amount,
    })?,
    funds: vec![],
});

let mint_freight_token_msg = CosmosMsg::Wasm(WasmMsg::Execute {
    contract_addr: validated_token_addr.to_string(),
    msg: to_binary(&cosmonaut_cw20::msg::ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount,
    })?,
    funds: vec![],
});
```

* ### query.rs
```rust
None => to_binary(&FreightTokenBalanceResponse {
    symbol,
    balance: Uint128::zero(),
}),
```
