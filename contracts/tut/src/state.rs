use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Account<'a> {
    pub balance: i32,                    // Total balance across all tokens
    pub debt: i32,                       // Total debt across all tokens
    pub owner: Addr,                      // Account owner
    pub deposits: Map<'a,String, i32>,  // Deposits by token address
    pub borrows: Map<'a,String, i32>,   // Borrowed amounts by token address
}

pub const ACCOUNT: Item<Account> = Item::new("account");
