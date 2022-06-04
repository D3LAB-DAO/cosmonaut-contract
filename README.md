## cw-multi-test used to test interaction among contracts

```shell
cargo test
```

## Build contracts
for apple silicon
```shell
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/workspace-optimizer-arm64:0.12.6
```
for other architectures
```shell
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/workspace-optimizer:0.12.6
```

## Workflow

1. Store cw20 for money, cw721 for nft(spaceship).
2. Instantiate the main contract. During this, money and spaceship contracts' addresses will be stored after
   instantiated.
3. Mint a spaceship with cw721. The main contract will be the owner.
4. Store and instantiate cw20 contracts for other freight to load to spaceship.
5. Buy money token with native token like uatom
6. Buy freight token with money token
7. Instantiate and execute 'AddFreightContract' to save info of them.
8. Increase allowance of money token for main contract to buy spaceship nft.
9. Execute buy nft.
10. Increase allowance of freight token for main contract to buy spaceship nft.
11. Execute load freight.
12. Query balance of money and freight tokens.
13. Play game with pseudo-random

