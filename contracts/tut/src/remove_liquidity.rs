use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg, BankMsg,
};
use cw_storage_plus::{Item};
use serde::{Deserialize, Serialize};

// ============================ Structs for Queries and State ============================
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct PoolInfo {
    pub vlp: String,
    pub token_1: String,
    pub token_2: String,
}

// Contract state to store user LP balances
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserLPBalance {
    pub user: Addr,
    pub vlp_address: String,
    pub lp_token_amount: Uint128,
}

const USER_LP_BALANCES: Item<UserLPBalance> = Item::new("user_lp_balances");

// ============================ Instantiate Function ============================
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = UserLPBalance {
        user: info.sender.clone(),
        vlp_address: String::new(),
        lp_token_amount: Uint128::zero(),
    };
    USER_LP_BALANCES.save(deps.storage, &state)?;
    Ok(Response::new().add_attribute("action", "instantiate"))
}

// ============================ Execute Messages ============================

// WithdrawLiquidityMsg struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WithdrawLiquidityMsg {
    pub vlp_address: String,
    pub lp_allocation: Uint128,
    pub timeout: u64,
    pub cross_chain_addresses: Vec<String>,
}

// Execute function to withdraw liquidity
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: WithdrawLiquidityMsg,
) -> Result<Response, ContractError> {
    // Retrieve user's LP balance from state
    let user_balance = USER_LP_BALANCES.load(deps.storage)?;

    // Ensure the user has enough LP tokens to withdraw
    if user_balance.lp_token_amount < msg.lp_allocation {
        return Err(ContractError::InsufficientLPTokens {});
    }

    // Perform the cross-chain liquidity removal using REST API call
    let tx_result = remove_liquidity(deps, info, msg.vlp_address.clone(), msg.lp_allocation, msg.timeout, msg.cross_chain_addresses)?;

    // Update user's LP balance
    USER_LP_BALANCES.update(deps.storage, |mut state| -> StdResult<_> {
        state.lp_token_amount -= msg.lp_allocation;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_message(tx_result)
        .add_attribute("action", "withdraw_liquidity")
        .add_attribute("from", info.sender)
        .add_attribute("amount", msg.lp_allocation.to_string()))
}

// ============================ Liquidity Removal Function ============================
pub fn remove_liquidity(
    deps: DepsMut,
    info: MessageInfo,
    vlp_address: String,
    lp_allocation: Uint128,
    timeout: u64,
    cross_chain_addresses: Vec<String>,
) -> StdResult<WasmMsg> {
    let rest_endpoint = "https://euclid-layer.rest/api"; // Example REST endpoint for Euclid Layer

    // Create REST call message for liquidity removal
    let msg = WasmMsg::Execute {
        contract_addr: rest_endpoint.to_string(),
        msg: to_binary(&{
            let rest_payload = serde_json::json!({
                "vlp_address": vlp_address,
                "lp_allocation": lp_allocation,
                "sender": {
                    "address": info.sender.to_string(),
                    "chain_uid": "chain-uid-placeholder"
                },
                "timeout": timeout,
                "cross_chain_addresses": cross_chain_addresses,
            });
            rest_payload
        })?,
        funds: vec![],
    };

    Ok(msg)
}

// ============================ Queries ============================

// Query to get the available pools from Euclid Layer
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryPoolsMsg;

pub fn query_pools(_deps: Deps) -> StdResult<Binary> {
    let query = "query All_vlps { router { all_vlps { vlps { vlp token_1 token_2 } } } }";
    // Implement pagination if needed
    let pools: Vec<PoolInfo> = vec![
        PoolInfo {
            vlp: "nibi1gd52zq92wexpv0jwzktkma5krxzgkuupqlltx6qxqv4u8xp39d3qa5jdxk".to_string(),
            token_1: "osmo".to_string(),
            token_2: "sol".to_string(),
        },
        PoolInfo {
            vlp: "nibi1x34fkar6puj4v3n0h84xf5y03u3uw5z5ggk95asql2cey6u8pygqlfups2".to_string(),
            token_1: "osmo".to_string(),
            token_2: "usdt".to_string(),
        },
    ];

    to_binary(&pools)
}

// Query user's LP balance
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryUserBalanceMsg {
    pub user_address: String,
}

pub fn query_user_balance(deps: Deps, msg: QueryUserBalanceMsg) -> StdResult<Binary> {
    let user = USER_LP_BALANCES.load(deps.storage)?;

    if user.user == Addr::unchecked(msg.user_address) {
        to_binary(&user)
    } else {
        Err(StdError::generic_err("User not found"))
    }
}

// ============================ Error Handling ============================
#[derive(Debug, thiserror::Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] cosmwasm_std::StdError),

    #[error("Insufficient LP Tokens")]
    InsufficientLPTokens {},
}

// ============================ Tests ============================
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Uint128;

    #[test]
    fn test_withdraw_liquidity() {
        let mut deps = mock_dependencies();
        let info = mock_info("sender", &[]);

        // Instantiate contract
        let msg = InstantiateMsg {};
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // Execute withdrawal
        let withdraw_msg = WithdrawLiquidityMsg {
            vlp_address: "vlp_address".to_string(),
            lp_allocation: Uint128::new(1000),
            timeout: 3600,
            cross_chain_addresses: vec!["osmo_address".to_string(), "sol_address".to_string()],
        };

        let res = execute(deps.as_mut(), mock_env(), info, withdraw_msg);
        assert!(res.is_ok());
    }
}
