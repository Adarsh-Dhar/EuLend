use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

use cosmwasm_std::Uint128;
use pyth_sdk_cw::PriceIdentifier;
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



#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Oracle {
    // Available price feeds and their ids are listed in pyth-sdk-cw Readme.
    pub price_feed_id:      PriceIdentifier,
    // Contract address of Pyth in different networks are listed in pyth-sdk-cw Readme.
    pub pyth_contract_addr: Addr,
}

pub const ORACLE: Item<Oracle> = Item::new("oracle");



pub const ACCOUNT_STORAGE: AccountStorage = AccountStorage {
    deposits: Map::new("deposits"),
    borrows: Map::new("borrows"),
};

pub const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
