use substreams::Hex;
use substreams_ethereum::rpc::RpcBatch;

use crate::abi::erc4626::functions;
use crate::pb::erc4626::{Erc4626Asset};

pub fn get_erc4626_token(token_address: &String) -> Option<Erc4626Asset> {
    let batch = RpcBatch::new();
    let responses = batch
        // ERC20
        .add(
            functions::Name {},
            hex::decode(token_address).unwrap(),
        )
        .add(
            functions::Symbol {},
            hex::decode(token_address).unwrap(),
        )
        .add(
            functions::Decimals {},
            hex::decode(token_address).unwrap(),
        )
        // .add(
        //     functions::TotalSupply {},
        //     hex::decode(token_address).unwrap(),
        // )
        //     // ERC4626
        .add(
            functions::Asset {},
            hex::decode(token_address).unwrap(),
        )
        .add(
            functions::TotalAssets {},
            hex::decode(token_address).unwrap(),
        )
        .execute()
        .unwrap()
        .responses;


    let name: String;
    match RpcBatch::decode::<_, functions::Name>(&responses[0]) {
        Some(decoded_name) => {
            name = decoded_name;
        }
        None => {
            log::debug!(
                "{} is not a ERC4626 token contract - name `eth_call` failed",
                &token_address,
            );
            return None;
        }
    };

    let symbol: String;
    match RpcBatch::decode::<_, functions::Symbol>(&responses[1]) {
        Some(decoded_symbol) => {
            symbol = decoded_symbol;
        }
        None => {
            log::debug!(
                "{} is not a ERC4626 token contract - symbol `eth_call` failed",
                &token_address,
            );
            return None;
        }
    };

    let decimals: u64;
    match RpcBatch::decode::<_, functions::Decimals>(&responses[2]) {
        Some(decoded_decimals) => {
            decimals = decoded_decimals.to_u64();
        }
        None => {
            log::debug!(
                "{} is not a ERC4626 token contract - decimal `eth_call` failed",
                Hex(&token_address),
            );

            return None;
        }
    };

    // let total_supply: u64;
    // match RpcBatch::decode::<_, functions::TotalSupply>(&responses[3]) {
    //     Some(decoded_total_supply) => {
    //         total_supply = decoded_total_supply.to_u64();
    //     }
    //     None => {
    //         log::debug!(
    //             "{} is not a ERC4626 token contract - total supply `eth_call` failed",
    //             Hex(&token_address),
    //         );
    //
    //         return None;
    //     }
    // };

    let asset: String;
    match RpcBatch::decode::<_, functions::Asset>(&responses[3]) {
        Some(decoded_asset) => {
            asset = Hex(decoded_asset).to_string();
        }
        None => {
            log::debug!(
                "{} is not a ERC4626 token contract - asset `eth_call` failed",
                Hex(&token_address),
            );

            return None;
        }
    };

    let total_assets: u64;
    match RpcBatch::decode::<_, functions::TotalAssets>(&responses[4]) {
        Some(decoded_total_assets) => {
            total_assets = decoded_total_assets.to_u64();
        }
        None => {
            log::debug!(
                "{} is not a ERC4626 token contract - total assets `eth_call` failed",
                Hex(&token_address),
            );

            return None;
        }
    };

    return Some(Erc4626Asset {
        id: token_address.clone(),
        address: token_address.clone(),
        name,
        symbol,
        decimals,
        // total_supply,
        asset,
        total_assets,
    });
}