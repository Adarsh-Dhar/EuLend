use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq,JsonSchema)]
pub struct Account {
    pub id: i32,
    pub balance: f64,
    pub debt: f64,
    pub owner: Addr,
}

// Define storage for deposits and borrows separately
pub struct AccountStorage<'a> {
    pub deposits: Map<'a, String, f64>,  // Deposits by token address
    pub borrows: Map<'a, String, f64>,   // Borrows by token address
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EuclidAddLiquidityMsg {
    pub token1: String,
    pub token2: String,
    pub amount1: f64,
    pub amount2: f64,
    slippage_tolerance: u64,
}

// Storage structure for the Lending Pool
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PoolState {
    pub owner: Addr,
    pub total_deposits: f64,
    pub total_borrowed: f64,
    pub interest_rate: f64,
}

// Storage structure for individual user positions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserPosition {
    pub owner: Addr,
    pub deposit: f64,
    pub debt: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendingPoolParams {
    pub asset_token: Addr,
    pub interest_rate_model: InterestRateModel,
    pub reserve_factor: u64,
    pub liquidation_threshold: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InterestRateModel {
    pub base_rate: u64,
    pub slope1: u64,
    pub slope2: u64,
    pub optimal_utilization: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendingPoolsResponse {
    pub lending_pools: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub lending_pools: Vec<Addr>,
    pub lending_pool_code_id: u64,
    pub prices: Vec<(Addr, f64)>,
    pub count : f64,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub price: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimulatedSwap {
    pub amount_out: f64,
    pub amount_in: f64,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State {
//     pub prices: Vec<(Addr, f64)>,
// }

pub const ACCOUNT: Item<Account> = Item::new("account");


pub const POOL_STATE: Item<PoolState> = Item::new("pool_state");
pub const USER_POSITIONS: Item<UserPosition> = Item::new("user_positions");
pub const STATE: Item<State> = Item::new("state");
pub const SIMULATED_SWAP: Item<SimulatedSwap> = Item::new("simulated_swap");

pub const ACCOUNT_STORAGE: AccountStorage = AccountStorage {
    deposits: Map::new("deposits"),
    borrows: Map::new("borrows"),
};

