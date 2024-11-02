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



// Collaterals by token denomination
pub const COLLATERAL: Map<&str, Uint128> = Map::new("collateral");
pub const ACCOUNTS: Map<&str, Account> = Map::new("accounts");
pub const ESCROW : Uint128 = Uint128::zero();
pub const LIQUIDITY_PROVIDERS: Map<&Addr, LiquidityProvider> = Map::new("liquidity_providers");
