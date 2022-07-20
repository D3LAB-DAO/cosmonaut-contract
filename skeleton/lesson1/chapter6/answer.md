* ### state.rs

q1)
```rust
pub type Extension = Metadata;
```

q2)
```rust
pub freights: Vec<Freight>,
```

* ### msg.rs

q3)
```rust
Mint(MintMsg<Extension>),
```

q4)
```rust
ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => Ok(Cw721ExecuteMsg::TransferNft {
                recipient,
                token_id,
            }),
```

* ### contract.rs

q5)
```rust
cw721_contract.instantiate(deps, env, info.clone(), msg)?;
```

* ### execute.rs

q6)
```rust
let execute_res = self.execute(deps, env, info, cw721_msg);
```

q7)
```rust
let candidate_idx = extension.freights.iter().position(|l| l.denom == denom);
```

q8)
```rust
if extension.freights[idx].amount.u128() - amount.u128() == 0 {
    extension.freights.remove(idx);
} else {
    extension.freights[idx].amount =
    extension.freights[idx].amount.checked_sub(amount).unwrap();
}
```
