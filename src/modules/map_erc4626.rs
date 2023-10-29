// use substreams_ethereum::block_view::LogView;
// use substreams_ethereum::Event;
// use substreams_ethereum::pb::eth;
//
// use crate::abi::erc4626;
//
// use crate::pb::erc4626::{Transfer, Transfers};
// use crate::utils::{format_bigint_u64, format_hex};
//
// const aUSDT = 0x23878914efe38d27c4d67ab83ed1b93a74d4086a;
// const CRV = 0xd533a949740bb3306d119cc777fa900ba034cd52;
// const stETH = 0xae7ab96520de3a18e5e111b5eaab095312d7fe84;
//
// pub fn format_erc4626(data: erc4626::events::Transfer, log: LogView) -> Transfer {
//     Transfer {
//         id: format_hex(&log.receipt.transaction.hash),
//         token_address: format_hex(&log.address()),
//         value: format_bigint_u64(data.value),
//     }
// }
//
// #[substreams::handlers::map]
// fn map_erc4626(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
// //     let transfers: Vec<Transfer> = block.transactions()
// //         .map(|log| {
// //
// //             erc4626::events::Transfer::match_and_decode(log).map(|transfer| {
// //                 format_erc4626(transfer, log)
// //             })
// //         })
// //         .collect();
// //
// //     Ok(Transfers { transfers })
//
//
// //     let mut balance_changes = Vec::new();
//
//     for trx in block.transaction_traces.iter() {
//         if trx.status != TransactionTraceStatus::Succeeded as i32 {
//             continue;
//         }
//
//         for call in trx.calls.iter() {
//             if call.state_reverted {
//                 continue;
//             }
//
//             for log in call.logs.iter() {
//                 let transfer = match Transfer::match_and_decode(log) {
//                     Some(transfer) => transfer,
//                     None => continue,
//                 };
//
//                 if transfer.value.is_zero() {
//                     continue;
//                 }
//             }
//         }
//     }
// }