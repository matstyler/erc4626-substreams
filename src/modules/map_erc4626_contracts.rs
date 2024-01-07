use substreams::Hex;
use substreams_ethereum::pb::eth;
use crate::erc4626::get_erc4626_token;

use crate::pb::erc4626::{Erc4626Assets};

#[substreams::handlers::map]
fn map_erc4626_contracts(block: eth::v2::Block) -> Result<Erc4626Assets, substreams::errors::Error> {
    let erc4626_assets = block
        .transactions()
        .flat_map(|tx|
            tx.calls
                .iter()
                .filter(|call| !call.state_reverted)
                .filter(|call| call.call_type == eth::v2::CallType::Create as i32)
                // check if contract is a erc4626 compatible
                .filter_map(|call| get_erc4626_token(&Hex(call.address.clone()).to_string().clone()))
        )
        .collect();
    Ok(Erc4626Assets { erc4626_assets })
}