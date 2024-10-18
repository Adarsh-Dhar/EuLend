use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use cosmwasm_std::Uint128;
use pyth_sdk_cw::PriceIdentifier;


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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EuclidAddLiquidityMsg {
    pub token1: String,
    pub token2: String,
    pub amount1: Uint128,
    pub amount2: Uint128,
    slippage_tolerance: Uint128,
}

// Storage structure for the Lending Pool
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PoolState {
    pub owner: Addr,
    pub total_deposits: Uint128,
    pub total_borrowed: Uint128,
    pub interest_rate: Uint128,
}

// Storage structure for individual user positions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserPosition {
    pub owner: Addr,
    pub deposit: Uint128,
    pub debt: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendingPoolParams {
    pub asset_token: Addr,
    pub interest_rate_model: InterestRateModel,
    pub reserve_factor: Uint128,
    pub liquidation_threshold: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InterestRateModel {
    pub base_rate: Uint128,
    pub slope1: Uint128,
    pub slope2: Uint128,
    pub optimal_utilization: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LendingPoolsResponse {
    pub lending_pools: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub lending_pools: Vec<Addr>,
    pub lending_pool_code_id: Uint128,
    pub prices: Vec<(Addr, Uint128)>,
    pub count : Uint128,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub price: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SimulatedSwap {
    pub amount_out: Uint128,
    pub amount_in: Uint128,
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State {
//     pub prices: Vec<(Addr, Uint128)>,
// }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Oracle {
    // Available price feeds and their ids are listed in pyth-sdk-cw Readme.
    pub price_feed_id:      PriceIdentifier,
    // Contract address of Pyth in different networks are listed in pyth-sdk-cw Readme.
    pub pyth_contract_addr: Addr,
}

pub const ORACLE: Item<Oracle> = Item::new("oracle");

pub const ACCOUNT: Item<Account> = Item::new("account");


pub const POOL_STATE: Item<PoolState> = Item::new("pool_state");
pub const USER_POSITIONS: Item<UserPosition> = Item::new("user_positions");
pub const STATE: Item<State> = Item::new("state");
pub const SIMULATED_SWAP: Item<SimulatedSwap> = Item::new("simulated_swap");

pub const ACCOUNT_STORAGE: AccountStorage = AccountStorage {
    deposits: Map::new("deposits"),
    borrows: Map::new("borrows"),
};

