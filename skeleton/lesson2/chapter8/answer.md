* ### contract.rs
q1)
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenExtension {
    pub unit_weight: Uint128,
}
```

q2)
```rust
ExecuteMsg::Transfer { recipient, amount } => {
    execute_transfer(deps, env, info, recipient, amount)
}
ExecuteMsg::Burn { amount } => execute_burn(deps, env, info, amount),
```

* ### execute.rs

q3)
```rust
if mint.minter != info.sender {
return Err(ContractError::Unauthorized {});
}
```
q4)
```rust
config.mint = Some(minter_data);
TOKEN_INFO.save(deps.storage, &config)?;
```

* ### query.rs
q5)
```rust
let token_extension = TOKEN_EXTENSION.load(deps.storage)?;

Ok(TokenExtension {
    unit_weight: token_extension.unit_weight,
})
```
