```shell
docker exec -it wasmd wasmd tx wasm instantiate 3 \
'{"money_cw20_contract":{"addr":"","code_id":1},"spaceship_cw721_contract":{"addr":"","code_id":2}}' \
 --label "mars token" --admin wasm1ccrvne2lz6funpxxkk6yumc8ll4l3c2e3kkgj4 --amount 10000ucosm \
 --from validator --chain-id testing --gas-prices 0.1ucosm --gas auto --gas-adjustment 1.3 -b block -y
```
