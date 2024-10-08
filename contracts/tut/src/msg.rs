use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit { amount: i32 },
    Withdraw { amount: i32 },
    ReceiveMessageEvm {
        source_chain: String,
        source_address: String,
        payload: Binary,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetBalanceResponse)]
    GetBalance {},
}

// We define a custom struct for each query response

#[cw_serde]
pub struct GetBalanceResponse {
    pub balance: i32,
}
