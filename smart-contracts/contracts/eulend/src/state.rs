use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Account {
    pub address: String,
    pub borrowed_usdc: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidityProvider {
    pub address: String,
    pub liquidity_amount: Uint128,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Collateral {
    pub address: String,
    pub token_denom: String,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum PoolUtilization {
    High,
    Medium,
    Low,
}


impl ToString for PoolUtilization {
    fn to_string(&self) -> String {
        match self {
            PoolUtilization::Low => "low".to_string(),
            PoolUtilization::Medium => "medium".to_string(),
            PoolUtilization::High => "high".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Pool {
    pub id : String,
    pub lend_token: String,
    pub borrow_token: String,
    pub utilization: PoolUtilization,
    pub total_liquidity: Uint128,
    pub total_borrowed: Uint128,
}



// Collaterals by token denomination
pub const COLLATERAL: Item<Collateral> = Item::new("collateral");
pub const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
pub const ESCROW: Uint128 = Uint128::zero();
pub const LIQUIDITY_PROVIDERS: Item<LiquidityProvider> = Item::new("liquidity_providers");
pub const POOLS: Map<&str, Pool> = Map::new("pools");
