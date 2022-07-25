* ### execute.rs

q1)
```rust
let nft_info: NftInfoResponse<Metadata> = deps.querier.query_wasm_smart(
    config.spaceship_cw721_contract.as_ref().to_string(),
    &Cw721QueryMsg::NftInfo {
        token_id: token_id.clone(),
    },
)?;
```

q2)
```rust
let total_freight_weight: u128 = nft_info
    .extension
    .freights
    .iter()
    .map(|f| f.unit_weight.u128() * f.amount.u128())
    .sum();
```

q3)
```rust
timestamp_int_nanos.rem(Uint128::new(MAX_FREIGHT_WEIGHT))
```

q4)
```rust
_ in 0..epoch.u128()
```

q5)
```rust
let burn_fuel_msg = Cw721ExecuteMsg::BurnFuel {
        token_id,
        amount: Uint128::new(FUEL_PER_GAME)
            .checked_mul(epoch)
            .map_err(StdError::overflow)?,
    };
```
