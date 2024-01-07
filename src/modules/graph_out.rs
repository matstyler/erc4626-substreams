use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;
use crate::pb::erc4626::{Erc4626Assets};

#[substreams::handlers::map]
pub fn graph_out(assets: Erc4626Assets) -> Result<EntityChanges, substreams::errors::Error> {
    // hash map of name to a table
    let mut tables = Tables::new();

    for erc4626 in assets.erc4626_assets.into_iter() {
        tables
            .create_row("ERC4626", erc4626.id.clone())
            .set("address", erc4626.address)
            .set("name", erc4626.name)
            .set("symbol", erc4626.symbol)
            .set("decimals", erc4626.decimals)
            // .set("totalSupply", erc4626.total_supply);
            .set("asset", erc4626.asset)
            .set("totalAssets", erc4626.total_assets);
    }

    Ok(tables.to_entity_changes())
}