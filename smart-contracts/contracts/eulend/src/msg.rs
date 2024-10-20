use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Binary, Coin};
use std::time::Duration;



#[cw_serde]
pub enum InstantiateMsg {}


#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        token_address: String,
        amount: Uint128,
        account_id: Uint128,
    },
    Withdraw {
        token_address: String,
        amount: Uint128,
        account_id: Uint128,

    },
    Borrow {
        borrow_token_address: String,
        amount: Uint128,
        collateral_token_address: String,
        account_id: Uint128,

    },
    Repay {
        token_address: String,
        amount: Uint128,
        account_id: Uint128,

    },
    
}

#[cw_serde]
pub enum QueryMsg {
    GetOraclePrice {
        token_main: String,
        token_ref: String,
    },
}





