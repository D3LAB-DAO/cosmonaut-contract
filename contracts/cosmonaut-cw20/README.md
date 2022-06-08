# Start Wasmd

```shell
git clone https://github.com/CosmWasm/wasmd.git

git checkout v0.27.0

docker build -t cosmwasm/wasmd:latest .

docker run --rm -it \
    -e PASSWORD=xxxxxxxxx \
    --mount type=volume,source=wasmd_data,target=/root \
    cosmwasm/wasmd:latest /opt/setup_wasmd.sh cosmos1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6

docker run --name wasmd --rm -it -p 26657:26657 -p 26656:26656 -p 1317:1317 \
--mount type=volume,source=wasmd_data,target=/root \
cosmwasm/wasmd:latest /opt/run_wasmd.sh
```
# Add new wallet
```shell
docker exec -it wasmd wasmd keys add wallet
```

# Build wasmd file with rust-optimizer
```shell
cargo wasm
```
* ## For Apple Silicon
```shell
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.6
```

* ## For other architectures
```shell
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6
```

# Copy compiled wasm file into docker container
```shell
docker cp artifacts/$WAWM_FILE wasmd:/cosmonaut_cw20.wasm
```

# Store wasm file into docker container
```shell
docker exec -it wasmd wasmd tx wasm store "/cosmonaut_cw20.wasm" --gas-prices 0.1ucosm \
--gas auto --gas-adjustment 1.3 -y -b block --chain-id testing --from validator --output json
```
enter password that you set as environment value PASSWORD for docker (-e option), for this example code: "xxxxxxxxx"

# Instantiate wasm file
```shell
docker exec -it wasmd wasmd tx wasm instantiate $CODE_ID \
'{"name":"mars","symbol":"MARS","decimals":6,"initial_balances":[{"address":"wasm1ckdm8uaz5fhhs84z6vr6yheuf0yjlfjc63ejy9","amount":"1000"}],"mint":{"minter":"wasm1ckdm8uaz5fhhs84z6vr6yheuf0yjlfjc63ejy9","cap":"10000"}}' \
 --label "mars token" --admin wasm1ckdm8uaz5fhhs84z6vr6yheuf0yjlfjc63ejy9 --amount 10000ucosm --from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```

# Execute Transfer
```shell
docker exec -it wasmd wasmd tx wasm execute wasm1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3sq29c3m \
'{"transfer": {"recipient": "wasm19q25juan7y8s2dyrmmr2kcwdnew79p8tszej75", "amount": "100"}}' \
 --from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```

# Execute Increase Allowance
```shell
docker exec -it wasmd wasmd tx wasm execute wasm1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3sq29c3m \
'{"increase_allowance":{"spender": "wasm19q25juan7y8s2dyrmmr2kcwdnew79p8tszej75", "amount": "50"}}' \
--from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```

# Execute TransferFrom
```shell
docker exec -it wasmd wasmd tx wasm execute wasm1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3sq29c3m \
'{"transfer_from": {"owner": "wasm1ckdm8uaz5fhhs84z6vr6yheuf0yjlfjc63ejy9", "recipient": "wasm1868jmrxr6pqrtqqhzxn56pxad383qz77w8h9fe", "amount": "50"}}' \
--from wallet --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```

# Query Balance
```shell
docker exec -it wasmd wasmd query wasm contract-state smart wasm1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3sq29c3m \
'{"balance": {"address": "wasm19q25juan7y8s2dyrmmr2kcwdnew79p8tszej75"}}'
```
