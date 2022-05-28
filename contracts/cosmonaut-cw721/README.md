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
docker cp artifacts/$WAWM_FILE wasmd:/cosmonaut_cw721.wasm
```

# Store wasm file into docker container
```shell
docker exec -it wasmd wasmd tx wasm store "/cosmonaut_cw721.wasm" \
 --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -y -b block \
 --chain-id testing --from validator --output json
```
enter password that you set as environment value PASSWORD for docker (-e option), for this example code: "xxxxxxxxx"

# Instantiate wasm file
```shell
docker exec -it wasmd wasmd tx wasm instantiate 1 \
'{"name": "spaceship", "symbol":"SPACE", "minter": "wasm1rsqfxydrwdytw8k27vfa62m55yy0l42u88qd99"}' \
 --label "mars spaceship nft" --admin wasm1rsqfxydrwdytw8k27vfa62m55yy0l42u88qd99 --amount 10000ucosm \
  --from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```

```shell
docker exec -it wasmd wasmd tx wasm execute wasm18v47nqmhvejx3vc498pantg8vr435xa0rt6x0m6kzhp6yuqmcp8s9jfa9t \
 '{"mint":{"token_id":"1","owner":"wasm1unyuj8qnmygvzuex3dwmg9yzt9alhvyeat0uu0jedg2wj33efl5qqadsm8","token_uri":"","metadata":{"unit_denom":"mars","price":1000,"image":"image","image_data":"image_data","external_url":"external_url","description":"description","name":"name","attributes":"","background_color":"background_color","animation_url":"animation_url","youtube_url":"youtube_url"}}}' \
  --from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```


