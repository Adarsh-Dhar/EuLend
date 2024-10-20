use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

use cosmwasm_std::Uint128;

use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct Account {
    pub id: Uint128,
    pub balance: Uint128,
    pub debt: Uint128,
    pub owner: Addr,
}

// Define storage for deposits and borrows separately
pub struct AccountStorage<'a> {
    pub deposits: Map<'a, String, Uint128>,  // Deposits by token address
    pub borrows: Map<'a, String, Uint128>,   // Borrows by token address
}




pub const ACCOUNT_STORAGE: AccountStorage = AccountStorage {
    deposits: Map::new("deposits"),
    borrows: Map::new("borrows"),
};

pub const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
pub const ORACLE_ADDRESS: &str = "archway1909sv5amdvv5knskc2lmxetue3u3wyv8hzqcfdmw4jnkzryrllaqzydthg" ;
