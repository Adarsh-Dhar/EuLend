use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Binary, Coin};
use std::time::Duration;



#[cw_serde]
pub struct InstantiateMsg {}


#[cw_serde]
pub enum ExecuteMsg {
    CreateAccount {},
    Borrow {collateral_denom: String, collateral_amount: Uint128},
    Repay {withdraw_denom: String, withdraw_amount: Uint128}
    
}


// #[cw_serde]
// pub enum QueryMsg {
//     MaxWithdrawableAmount {token_denom : String},
//     GetAccount {address: String},
// }







