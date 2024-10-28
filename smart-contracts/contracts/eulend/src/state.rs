use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

use cosmwasm_std::Uint128;

use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct Account {
    pub address : String,
    pub borrowed_usdc: Uint128,
}

    // Collaterals by token address
pub const COLLATERAL = Map<String, Uint128>, 


pub const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
