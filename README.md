# cw-multi-test used to test interaction among contracts

```shell
cargo test
```

# Workflow

1. Store cw20 for money, cw721 for nft(spaceship).
2. Instantiate the main contract. During this, money and spaceship contracts' addresses will be stored after
   instantiated.
3. Mint a spaceship with cw721. The main contract will be the owner.
4. Store and instantiate cw20 contracts for other luggage to load to spaceship.
5. Increase allowance of money token for main contract to buy spaceship nft.
6. Execute buy nft.
7. Increase allowance of luggage token for main contract to buy spaceship nft.
8. Execute load luggage.
9. Query balance of money and luggage tokens.
