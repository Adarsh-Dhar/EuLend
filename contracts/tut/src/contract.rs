#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin};
use cw2::set_contract_version;
use cw20::{Cw20ExecuteMsg};
use cw_storage_plus::{ Map};

use crate::error::{ContractError};
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
        deposits: Map::new("new"),
        borrows: Map::new("new"),
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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {token_address, amount } => execute::deposit(deps, env,info, token_address, amount),
        ExecuteMsg::Withdraw { token_address,amount } => execute::withdraw(deps,env, info,token_address, amount),
        ExecuteMsg::Borrow { token_address, amount, collateral_token_address, collateral_amount } =>
        execute::borrow(deps,env, info, token_address, amount, collateral_token_address, collateral_amount),
        ExecuteMsg::Repay { token_address, amount } => execute::repay(deps,env, info, token_address, amount),
        ExecuteMsg::Liquidate { borrower, debt_token, collateral_token } => execute::liquidate(deps, env, info, borrower, debt_token, collateral_token),
        // ExecuteMsg::Increment {} => execute::increment(deps),
        // ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
    //     ExecuteMsg::ReceiveMessageEvm {
    //         source_chain,
    //         source_address,
    //         payload,
    //     } => exec::receive_message_evm(deps, source_chain, source_address, payload)
    // }
};

pub mod execute {
    use super::*;

    pub fn deposit(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: String,
        amount: i32,
    ) ->  Result<Response, ContractError> {
        let mut account = ACCOUNT.load(deps.storage)?;
        let depositor = info.sender;
    
        // Check if depositor is the owner
        if account.owner != depositor {
            return Err(ContractError::Unauthorized{});
        }
    
        // Update user's deposit for the specific token
        let current_deposit = account.deposits.get(&token_address).cloned().unwrap_or_default();
        account.deposits.insert(token_address.clone(), current_deposit + amount);
    
        // Update the total balance
        account.balance += amount;
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // If token is native, it's already sent with the transaction
        // If it's a cw20 token, we need to execute a transfer
        let transfer_msg = if token_address == "native" {
            None
        } else {
            Some(ExecuteMsg::Deposit {
                token_address: token_address.clone(),
                amount: amount,
            })
        };
    
        Ok(Response::new()
            .add_attribute("action", "deposit")
            .add_attribute("depositor", depositor)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount.to_string()))
    }
    

    pub fn withdraw(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: i32,
    ) -> Result<Response, ContractError> {
        let mut account = ACCOUNT.load(deps.storage)?;
        let withdrawer = info.sender;
    
        // Check if the withdrawer is the owner
        if account.owner != withdrawer {
            return Err(ContractError::Unauthorized{});
        }
    
        // Check if the user has enough balance for the specific token
        let current_deposit = account.deposits.get(&token_address).cloned().unwrap_or_default();
        if current_deposit < amount {
            return Err(ContractError::InsufficientFunds{});
        }
    
        // Update the user's deposit
        account.deposits.insert(token_address.clone(), current_deposit - amount);
        account.balance -= amount;
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // Transfer tokens to user
        // let transfer_msg = if token_address == "native" {
        //     ExecuteMsg::Bank {
        //         to_address: withdrawer.to_string(),
        //         amount: vec![Coin {
        //             denom: "uarch".to_string(),
        //             amount,
        //         }],
        //     }
        // } else {
        //     ExecuteMsg::Withdraw {
        //         token_address: token_address.clone(),
        //         amount: amount,
        //     }
        // };
    
        Ok(Response::new()
            .add_attribute("action", "withdraw")
            .add_attribute("withdrawer", withdrawer)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount.to_string()))
    }
    


    pub fn borrow(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: i32,
        collateral_token_address: String,
        collateral_amount: i32,
    ) -> Result<Response, ContractError>  {
        let mut account = ACCOUNT.load(deps.storage)?;
        let borrower = info.sender;
    
        // Check if borrower is the owner
        if account.owner != borrower {
            return Err(ContractError::Unauthorized{});
        }
    
        // Check if the borrower has enough collateral
        let current_collateral = account.deposits.get(&collateral_token_address).cloned().unwrap_or_default();
        if current_collateral < collateral_amount {
            return Err(ContractError::InsufficientFunds{});
        }
    
        // Update borrower's collateral and borrow
        account.deposits.insert(collateral_token_address.clone(), current_collateral - collateral_amount);
        let current_borrow = account.borrows.get(&token_address).cloned().unwrap_or_default();
        account.borrows.insert(token_address.clone(), current_borrow + amount);
        account.debt += amount;
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // Transfer borrowed tokens to user
        let transfer_msg = if token_address == "native" {
            ExecuteMsg::Send {
                to_address: borrower.to_string(),
                amount: vec![Coin {
                    denom: "uarch".to_string(),
                    amount,
                }],
            }
        } else {
            ExecuteMsg::Borrow {
                contract_addr: token_address.clone(),
                msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: borrower.to_string(),
                    amount,
                })?,
                funds: vec![],
            }
        };
    
        Ok(Response::new()
            .add_attribute("action", "borrow")
            .add_attribute("borrower", borrower)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount.to_string())
            .add_attribute("collateral_token", collateral_token_address)
            .add_attribute("collateral_amount", collateral_amount.to_string()))
    }
    

    pub fn repay(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        token_address: String,
        amount: i32,
    ) -> Result<Response, ContractError> {
        let mut account = ACCOUNT.load(deps.storage)?;
        let repayer = info.sender;
    
        // Check if repayer is the owner
        if account.owner != repayer {
            return Err(ContractError::generic_err("Unauthorized"));
        }
    
        // Update borrower's debt
        let current_borrow = account.borrows.get(&token_address).cloned().unwrap_or_default();
        if current_borrow < amount {
            return Err(ContractError::generic_err("Repayment amount exceeds debt"));
        }
        account.borrows.insert(token_address.clone(), current_borrow - amount);
        account.debt -= amount;
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // Handle token transfer
        let transfer_msg = if token_address == "native" {
            None
        } else {
            Some(ExecuteMsg::Repay {
                token_address: token_address.clone(),   
                amount : amount,
            })
        };
    
        Ok(Response::new()
            .add_attribute("action", "repay")
            .add_attribute("repayer", repayer)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount.to_string()))
    }
    

    pub fn liquidate(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        borrower: String,
        debt_token: String,
        collateral_token: String,
    ) -> Result<Response, ContractError> {
        let mut account = ACCOUNT.load(deps.storage)?;
        let liquidator = info.sender;
        let borrower_addr = deps.api.addr_validate(&borrower)?;
    
        // Get the borrowed amount and collateral
        let borrowed_amount = account.borrows.get(&debt_token).cloned().unwrap_or_default();
        let collateral_amount = account.deposits.get(&collateral_token).cloned().unwrap_or_default();
    
        // Update state
        account.borrows.remove(&debt_token);
        account.deposits.remove(&collateral_token);
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // Transfer debt tokens from liquidator to contract
        let debt_transfer_msg = ExecuteMsg::Execute {
            contract_addr: debt_token.clone(),
            msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
                owner: liquidator.to_string(),
                recipient: _env.contract.address.to_string(),
                amount: borrowed_amount,
            })?,
            funds: vec![],
        };
    
        // Transfer collateral to liquidator
        let collateral_transfer_msg = ExecuteMsg::Liquidate {
            borrower: borrower_addr.to_string(),
            debt_token: debt_token.clone(),
            collateral_token: collateral_token.clone(),
        };
    
        Ok(Response::new()
            .add_attribute("action", "liquidate")
            .add_attribute("liquidator", liquidator)
            .add_attribute("borrower", borrower)
            .add_attribute("debt_token", debt_token)
            .add_attribute("debt_amount", borrowed_amount.to_string())
            .add_attribute("collateral_token", collateral_token)
            .add_attribute("collateral_amount", collateral_amount.to_string()))
    }
    


    // pub fn receive_message_evm(
    //     deps: DepsMut,
    //     _source_chain: String,
    //     _source_address: String,
    //     payload: Binary,
    // ) -> Result<Response, ContractError> {
    //     // decode the payload
    //     // executeMsgPayload: [sender, message]
    //     let decoded = decode(
    //         &vec![ParamType::String, ParamType::String],
    //         payload.as_slice(),
    //     )
    //     .unwrap();

    //     // store message
    //     STORED_MESSAGE.save(
    //         deps.storage,
    //         &Message {
    //             sender: decoded[0].to_string(),
    //             message: decoded[1].to_string(),
    //         },
    //     )?;

    //     Ok(Response::new())
    // }

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


// fn receive_message(
//     _deps: DepsMut,
//     source_chain: String,
//     source_address: String,
//     payload: Binary,
// ) -> Result<Response, ContractError> {
//     // Decode and process the payload here
//     let message: String = String::from_utf8(payload.0)
//         .map_err(|_| ContractError::generic_err("Invalid payload"))?;
    
//     // Log the source_chain, source_address, and decoded message
//     Ok(Response::new()
//         .add_attribute("action", "receive_message")
//         .add_attribute("source_chain", source_chain)
//         .add_attribute("source_address", source_address)
//         .add_attribute("message", message))
// }

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
}
}
