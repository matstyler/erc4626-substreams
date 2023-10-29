mod pb;
mod abi;
mod modules;
mod utils;

use pb::erc4626::{Transfers};

use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use crate::pb::erc4626::{Transfer};

#[substreams::handlers::map]
pub fn graph_out(transfers: Transfers, transfer: Transfer) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for transfer in transfers.transfers.into_iter() {
        tables
            .create_row("ERC4626", transfer.id.clone())
            .set("amount", transfer.value);
            // .set();

        // address
        // symbol
        // platform
        // totalAssets
        // totalSupply

        // tables
        //     .update_row("Pool", Hex(TRACKED_POOL).to_string())
        //     .set("last_transfer", transfer.id.clone())
        //     .set("total_supply", pool.total_supply)
        //     .set("total_borrow", pool.total_borrow);
    }

    Ok(tables.to_entity_changes())
}
