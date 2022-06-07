use cosmwasm_std::Deps;
use cw721::Cw721QueryMsg::NftInfo;
use cw721::NftInfoResponse;
use cw_multi_test::BasicApp;
use cosmonaut_cw721::state::Extension;

pub fn query_nft_info(
    app: BasicApp,
    contract_addr: &str,
) -> BasicApp {
    let res: NftInfoResponse<Extension> = app.wrap().query_wasm_smart(contract_addr, &NftInfo {
        token_id: "1".to_string()
    }).unwrap();

    println!("{:?}", res);

    app
}
