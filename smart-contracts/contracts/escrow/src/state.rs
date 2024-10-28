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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct GetReferenceData {
    pub symbol_pair: (String, String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct ReferenceData {
    // Pair rate e.g. rate of BTC/USD
    pub rate: Uint128,
    // Unix time of when the base asset was last updated. e.g. Last update time of BTC in Unix time
    pub last_updated_base: Uint128,
    // Unix time of when the quote asset was last updated. e.g. Last update time of USD in Unix time
    pub last_updated_quote: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub coin_id: String,
    pub price: Uint128,
    pub last_updated: u64,
    pub verification_status: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PricesResponse {
    pub prices: Vec<PriceResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LastUpdatedResponse {
    pub last_updated: u64,
}


pub const ACCOUNT_STORAGE: AccountStorage = AccountStorage {
    deposits: Map::new("deposits"),
    borrows: Map::new("borrows"),
};

pub const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
pub const ORACLE_ADDRESS: &str = "archway1909sv5amdvv5knskc2lmxetue3u3wyv8hzqcfdmw4jnkzryrllaqzydthg" ;
pub const VERIFIED_PRICES: Map<&str, PriceResponse> = Map::new("verified_prices");
