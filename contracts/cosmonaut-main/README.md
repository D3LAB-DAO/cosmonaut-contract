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

## Store wasm files

```shell
docker exec -it wasmd wasmd tx wasm store "/cosmonaut_cw20.wasm" \
 --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -y -b block \
 --chain-id testing --from validator --output json
```

```shell
docker exec -it wasmd wasmd tx wasm store "/cosmonaut_cw721.wasm" \
--gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -y -b block \
--chain-id testing --from validator --output json
```

```shell
docker exec -it wasmd wasmd tx wasm store "/cosmonaut_main.wasm" \
--gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -y -b block \
--chain-id testing --from validator --output json
```

## Init contract
money cw20 contract and spaceship cw721 contracts will be instantiated automatically
```shell
docker exec -it wasmd wasmd tx wasm instantiate 3 \
'{"money_cw20_contract":{"addr":"","code_id":1},"spaceship_cw721_contract":{"addr":"","code_id":2}}' \
--label "main contract" --admin wasm1g88fguh42fndm09cqjx2sgma0auz25jjjexmfg  --amount 1000000ucosm \
--from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
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

