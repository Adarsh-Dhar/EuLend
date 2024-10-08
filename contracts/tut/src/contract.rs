#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetBalanceResponse, InstantiateMsg, QueryMsg};
use crate::state::{Account, ACCOUNT};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tut";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let account = Account {
        debt: 0,
        balance: 5,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    ACCOUNT.save(deps.storage, &account)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("balance", 0.to_string())
        .add_attribute("debt", 0.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { amount } => execute::deposit(deps, info, amount),
        ExecuteMsg::Withdraw { amount } => execute::withdraw(deps, info, amount),
        // ExecuteMsg::Increment {} => execute::increment(deps),
        // ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::ReceiveMessageEvm {
            source_chain,
            source_address,
            payload,
        } => exec::receive_message_evm(deps, source_chain, source_address, payload)
    }
}

pub mod execute {
    use super::*;

    pub fn deposit(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
    ) -> StdResult<Response> {
        let mut state = STATE.load(deps.storage)?;
        let depositor = info.sender;
        
        // Update user's deposit
        let deposit_key = (depositor.clone(), token_address.clone());
        let current_deposit = state.user_deposits.get(&deposit_key).cloned().unwrap_or_default();
        state.user_deposits.insert(deposit_key, current_deposit + amount);
        
        // Save updated state
        STATE.save(deps.storage, &state)?;

        // If token is native, it's already sent with the transaction
        // If it's a cw20 token, we need to execute a transfer
        let transfer_msg = if token_address == "native" {
            None
        } else {
            Some(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: token_address,
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: depositor.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount,
                })?,
                funds: vec![],
            }))
        };

        Ok(Response::new()
            .add_message(transfer_msg.unwrap_or_default())
            .add_attribute("action", "deposit")
            .add_attribute("depositor", depositor)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount))
    }

    pub fn withdraw(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
    ) -> StdResult<Response> {
        let mut state = STATE.load(deps.storage)?;
        let withdrawer = info.sender;
        
        // Check if user has enough balance
        let deposit_key = (withdrawer.clone(), token_address.clone());
        let current_deposit = state.user_deposits.get(&deposit_key).cloned().unwrap_or_default();
        if current_deposit < amount {
            return Err(StdError::generic_err("Insufficient balance"));
        }
        
        // Update user's deposit
        state.user_deposits.insert(deposit_key, current_deposit - amount);
        
        // Save updated state
        STATE.save(deps.storage, &state)?;

        // Transfer tokens to user
        let transfer_msg = if token_address == "native" {
            CosmosMsg::Bank(BankMsg::Send {
                to_address: withdrawer.to_string(),
                amount: vec![Coin {
                    denom: "uarch".to_string(),
                    amount,
                }],
            })
        } else {
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: token_address.clone(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: withdrawer.to_string(),
                    amount,
                })?,
                funds: vec![],
            })
        };

        Ok(Response::new()
            .add_message(transfer_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("withdrawer", withdrawer)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount))
    }


    pub fn borrow(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
        collateral_token_address: String,
        collateral_amount: Uint128,
    ) -> StdResult<Response> {
        let mut state = STATE.load(deps.storage)?;
        let borrower = info.sender;
        
        // Check if borrower has enough collateral
        let collateral_key = (borrower.clone(), collateral_token_address.clone());
        let current_collateral = state.user_deposits.get(&collateral_key).cloned().unwrap_or_default();
        if current_collateral < collateral_amount {
            return Err(StdError::generic_err("Insufficient collateral"));
        }
        
        // Update borrower's collateral and borrow
        state.user_deposits.insert(collateral_key, current_collateral - collateral_amount);
        let borrow_key = (borrower.clone(), token_address.clone());
        let current_borrow = state.user_borrows.get(&borrow_key).cloned().unwrap_or_default();
        state.user_borrows.insert(borrow_key, current_borrow + amount);
        
        // Save updated state
        STATE.save(deps.storage, &state)?;

        // Transfer borrowed tokens to user
        let transfer_msg = if token_address == "native" {
            CosmosMsg::Bank(BankMsg::Send {
                to_address: borrower.to_string(),
                amount: vec![Coin {
                    denom: "uarch".to_string(),
                    amount,
                }],
            })
        } else {
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: token_address.clone(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: borrower.to_string(),
                    amount,
                })?,
                funds: vec![],
            })
        };

        Ok(Response::new()
            .add_message(transfer_msg)
            .add_attribute("action", "borrow")
            .add_attribute("borrower", borrower)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount)
            .add_attribute("collateral_token", collateral_token_address)
            .add_attribute("collateral_amount", collateral_amount))
    }

    pub fn repay(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
    ) -> StdResult<Response> {
        let mut state = STATE.load(deps.storage)?;
        let repayer = info.sender;
        
        // Update borrower's debt
        let borrow_key = (repayer.clone(), token_address.clone());
        let current_borrow = state.user_borrows.get(&borrow_key).cloned().unwrap_or_default();
        if current_borrow < amount {
            return Err(StdError::generic_err("Repayment amount exceeds debt"));
        }
        state.user_borrows.insert(borrow_key, current_borrow - amount);
        
        // Save updated state
        STATE.save(deps.storage, &state)?;

        // If token is native, it's already sent with the transaction
        // If it's a cw20 token, we need to execute a transfer
        let transfer_msg = if token_address == "native" {
            None
        } else {
            Some(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: token_address,
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: repayer.to_string(),
                    recipient: env.contract.address.to_string(),
                    amount,
                })?,
                funds: vec![],
            }))
        };

        Ok(Response::new()
            .add_message(transfer_msg.unwrap_or_default())
            .add_attribute("action", "repay")
            .add_attribute("repayer", repayer)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount))
    }

    pub fn liquidate(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        borrower: String,
        debt_token: String,
        collateral_token: String,
    ) -> StdResult<Response> {
        let mut state = STATE.load(deps.storage)?;
        let liquidator = info.sender;
        let borrower_addr = deps.api.addr_validate(&borrower)?;
        
        // Check if the position is liquidatable
        // This would involve checking the current prices, calculating the health factor, etc.
        // For simplicity, we're not implementing the full liquidation logic here
        
        // Get the borrowed amount and collateral
        let borrow_key = (borrower_addr.clone(), debt_token.clone());
        let borrowed_amount = state.user_borrows.get(&borrow_key).cloned().unwrap_or_default();
        let collateral_key = (borrower_addr.clone(), collateral_token.clone());
        let collateral_amount = state.user_deposits.get(&collateral_key).cloned().unwrap_or_default();
        
        // Calculate liquidation amount (in a real implementation, this would be more complex)
        let liquidation_amount = borrowed_amount;
        let liquidation_collateral = collateral_amount;
        
        // Update state
        state.user_borrows.remove(&borrow_key);
        state.user_deposits.remove(&collateral_key);
        
        // Save updated state
        STATE.save(deps.storage, &state)?;

        // Transfer debt tokens from liquidator to contract
        let debt_transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: debt_token.clone(),
            msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: liquidator.to_string(),
                recipient: _env.contract.address.to_string(),
                amount: liquidation_amount,
            })?,
            funds: vec![],
        });

        // Transfer collateral to liquidator
        let collateral_transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: collateral_token.clone(),
            msg: to_binary(&Cw20ExecuteMsg::Transfer {
                recipient: liquidator.to_string(),
                amount: liquidation_collateral,
            })?,
            funds: vec![],
        });

        Ok(Response::new()
            .add_message(debt_transfer_msg)
            .add_message(collateral_transfer_msg)
            .add_attribute("action", "liquidate")
            .add_attribute("liquidator", liquidator)
            .add_attribute("borrower", borrower)
            .add_attribute("debt_token", debt_token)
            .add_attribute("debt_amount", liquidation_amount)
            .add_attribute("collateral_token", collateral_token)
            .add_attribute("collateral_amount", liquidation_collateral))
    }
}


    pub fn receive_message_evm(
        deps: DepsMut,
        _source_chain: String,
        _source_address: String,
        payload: Binary,
    ) -> Result<Response, ContractError> {
        // decode the payload
        // executeMsgPayload: [sender, message]
        let decoded = decode(
            &vec![ParamType::String, ParamType::String],
            payload.as_slice(),
        )
        .unwrap();

        // store message
        STORED_MESSAGE.save(
            deps.storage,
            &Message {
                sender: decoded[0].to_string(),
                message: decoded[1].to_string(),
            },
        )?;

        Ok(Response::new())
    }

    // pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    //     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //         state.count += 1;
    //         Ok(state)
    //     })?;

    //     Ok(Response::new().add_attribute("action", "increment"))
    // }

    // pub fn reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    //     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //         if info.sender != state.owner {
    //             return Err(ContractError::Unauthorized {});
    //         }
    //         state.count = count;
    //         Ok(state)
    //     })?;
    //     Ok(Response::new().add_attribute("action", "reset"))
    // }
}

fn receive_message(
    _deps: DepsMut,
    source_chain: String,
    source_address: String,
    payload: Binary,
) -> StdResult<Response> {
    // Decode and process the payload here
    let message: String = String::from_utf8(payload.0)
        .map_err(|_| StdError::generic_err("Invalid payload"))?;
    
    // Log the source_chain, source_address, and decoded message
    Ok(Response::new()
        .add_attribute("action", "receive_message")
        .add_attribute("source_chain", source_chain)
        .add_attribute("source_address", source_address)
        .add_attribute("message", message))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
        QueryMsg::GetBalance {} => to_json_binary(&query::balance(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn balance(deps: Deps) -> StdResult<GetBalanceResponse> {
        let account = ACCOUNT.load(deps.storage)?;
        Ok(GetBalanceResponse {
            balance: account.balance,
        })
    }

    // pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
    //     let state = STATE.load(deps.storage)?;
    //     Ok(GetCountResponse { count: state.count })
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_json_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
        let value: GetBalanceResponse = from_json_binary(&res).unwrap();
        assert_eq!(5, value.balance);
    }

    // #[test]
    // fn increment() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Increment {};
    //     let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // should increase counter by 1
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_json_binary(&res).unwrap();
    //     assert_eq!(18, value.count);
    // }

    // #[test]
    // fn reset() {
    //     let mut deps = mock_dependencies();

    //     let msg = InstantiateMsg { count: 17 };
    //     let info = mock_info("creator", &coins(2, "token"));
    //     let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    //     // beneficiary can release it
    //     let unauth_info = mock_info("anyone", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
    //     match res {
    //         Err(ContractError::Unauthorized {}) => {}
    //         _ => panic!("Must return unauthorized error"),
    //     }

    // only the original creator can reset the counter
    //     let auth_info = mock_info("creator", &coins(2, "token"));
    //     let msg = ExecuteMsg::Reset { count: 5 };
    //     let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

    //     // should now be 5
    //     let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
    //     let value: GetCountResponse = from_json_binary(&res).unwrap();
    //     assert_eq!(5, value.count);
    // }

    #[test]
    fn test_deposit() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Deposit { amount: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Deposit { amount: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
        let value: GetBalanceResponse = from_json_binary(&res).unwrap();
        assert_eq!(10, value.balance);
    }

    #[test]
    fn test_withdraw() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Withdraw { amount: 2 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Withdraw { amount: 2 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
        let value: GetBalanceResponse = from_json_binary(&res).unwrap();
        assert_eq!(3, value.balance);
    }

    #[test]
    fn test_receive_msg() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Withdraw { amount: 2 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Withdraw { amount: 2 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
        let value: GetBalanceResponse = from_json_binary(&res).unwrap();
        assert_eq!(3, value.balance);
    }
}
