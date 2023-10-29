use substreams::scalar::BigInt;
use substreams::Hex;
use substreams_ethereum::block_view::LogView;
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth;

use crate::abi::erc4626::events::Transfer as ERC4626TransferEvent;
use crate::abi::erc4626::functions;

use crate::pb::erc4626::{Transfer, Transfers, Contract, Contracts};
use crate::utils::{format_bigint_u64, format_hex};

pub fn format_erc4626_contract(data: ERC4626TransferEvent, log: LogView) -> Contract {
    Contract {
        address: format_hex(&log.address()),
        block_number: block.number,
        timestamp: block.timestamp_seconds().to_string(),
        ordinal: tx.begin_ordinal,
    }
}

// #[substreams::handlers::map]
// fn map_erc4626_deployments(block: eth::v2::Block) -> Result<Transfers, substreams::errors::Error> {
// //     let mut transfers = vec![];
// //
//     for log in block.calls() {
//         if let Some(transfer) = erc4626::events::Deposit::match_and_decode(log) {
//             transfers.push(format_erc4626_transfer(transfer, log));
//             continue;
//
//         }
//     }
//
// //     let transfers: Vec<Transfer> = block
// //         .events()
// //         .map(|log| {
// //
// //
// //             // erc4626::events::Transfer::match_and_decode(log).map(|transfer| {
// //             //     format_erc4626_transfer(transfer, log)
// //             // })
// //         })
// //         .collect();
//
//     Ok(Transfers { transfers })
// }

pub struct Erc4626Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub total_supply: BigInt,

    pub asset: String,
    pub total_assets: BigInt,

    // TODO: Do we need these?
    // pub max_deposit: BigInt,
    // pub max_withdraw: BigInt,
    // pub max_redeem: BigInt,
}

pub fn get_erc4626_token(token_address: String) -> Option<Erc4626Token> {
    let token_address_vec = Hex::decode(token_address.clone()).unwrap();

    // ERC20
    let name = functions::Name {}
        .call(token_address_vec.clone())
        .unwrap_or(String::new());
    let symbol = functions::Symbol {}
        .call(token_address_vec.clone())
        .unwrap_or(String::new());
    let decimals = functions::Decimals {}
        .call(token_address_vec.clone())
        .unwrap_or(BigInt::zero())
        .to_u64();
    let total_supply = functions::TotalSupply {}
        .call(token_address_vec.clone())
        .unwrap_or(BigInt::zero());

    // ERC4626
    let asset = functions::Asset {}
        .call(token_address_vec.clone())
        .call(|address| format_hex(&address))
        .unwrap_or(String::new());
    let total_assets = functions::TotalAssets {}
        .call(token_address_vec.clone())
        .unwrap_or(BigInt::zero());


    Some(Erc4626Token {
        address: token_address.clone(),
        name: name,
        symbol: symbol,
        decimals: decimals,
        total_supply: total_supply,

        asset: asset,
        total_assets: total_assets,
    })
}


#[substreams::handlers::map]
fn map_contracts(block: eth::v2::Block) -> Result<Contracts, substreams::errors::Error> {
    let contracts = block
        .transactions()
        .flat_map(|tx| {
            tx.calls
                .iter()
                .filter(|call| !call.state_reverted)
                .filter(|call| call.call_type == eth::v2::CallType::Create as i32)
                .filter(|call| call.call_type == eth::v2::CallType::Create as i32)
                .map(|call| {
                    for log in call.logs.iter() {
                        if let Some(event) = ERC4626TransferEvent::match_and_decode(log) {
                            return vec![new_erc20_transfer(hash, log.block_index, event)];
                        }

                        if let Some(event) = ERC721TransferEvent::match_and_decode(log) {
                            return vec![new_erc721_transfer(hash, log.block_index, event)];
                        }

                        if let Some(event) = ERC1155TransferSingleEvent::match_and_decode(log) {
                            return vec![new_erc1155_single_transfer(hash, log.block_index, event)];
                        }

                        if let Some(event) = ERC1155TransferBatchEvent::match_and_decode(log) {
                            return new_erc1155_batch_transfer(hash, log.block_index, event);
                        }
                    }
                })
        })
        .collect();
    Ok(Contracts { contracts })
}