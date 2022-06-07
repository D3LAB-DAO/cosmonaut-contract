use cosmwasm_std::{Addr, coin, Empty};
use cw_multi_test::{BasicApp, custom_app};

pub fn init_app(
    admin: &str
) -> BasicApp {
    let init_funds = vec![coin(5000, "uatom")];
    custom_app::<Empty, Empty, _>(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(admin), init_funds)
            .unwrap();
    })
}

// fn next_block(block: &mut BlockInfo) {
//     block.time = block.time.plus_seconds(5);
//     block.height += 1;
// }
