use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Account {
    pub balance: i32,
    pub debt : i32,
    pub owner: Addr,
}



pub const STATE: Item<State> = Item::new("state");
pub const ACCOUNT: Item<Account> = Item::new("account");

