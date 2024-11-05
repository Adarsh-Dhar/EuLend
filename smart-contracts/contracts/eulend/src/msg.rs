use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Binary, Coin};
use std::time::Duration;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateAccount {},
    Borrow {borrow_amount: Uint128, collateral_denom: String},
    Repay {withdraw_denom: String, withdraw_amount: Uint128},
    DeleteAccount {},
    ProvideLiquidity {},
    ChangePoolUtilization {id : Uint128}
}

#[cw_serde]
pub enum QueryMsg {
    GetAccount {address: String},
}







