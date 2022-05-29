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
5. Buy money token with native token like uatom
6. Instantiate and execute 'AddLuggageContract' to save info of them.
7. Increase allowance of money token for main contract to buy spaceship nft.
8. Execute buy nft.
9. Increase allowance of luggage token for main contract to buy spaceship nft.
10. Execute load luggage.
11. Query balance of money and luggage tokens.

## TODO

Buy other luggage tokens with cw20 money tokens
