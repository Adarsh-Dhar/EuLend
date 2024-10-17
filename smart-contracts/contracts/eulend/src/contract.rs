#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Decimal};
use cw2::set_contract_version;
use cw_storage_plus::{Item};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{STATE, Account, ACCOUNT,ACCOUNT_STORAGE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:backend";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const ID_COUNTER: Item<Uint128> = Item::new("id_counter");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Load the current ID counter, or initialize it to 0 if it doesn't exist
    let mut current_id = match ID_COUNTER.may_load(deps.storage)? {
        Some(id) => id,
        None => Uint128::new     (0),
    };

    // Create a new Account with an empty `deposits` and `borrows` Map
    let account = Account {
        id: current_id,
        balance: Uint128::new(0),
        debt: Uint128::new(0),
        owner: info.sender.clone(),
    };

    // Save the new account
    ACCOUNT.save(deps.storage, &account)?;

    // Increment the ID counter for the next account
    current_id += Uint128::new(1);
    ID_COUNTER.save(deps.storage, &current_id)?;

    // Set contract version (or any other state if necessary)
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Return a success response with attributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("id", account.id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { token_address, amount } => {
            execute::deposit(deps, _env, info, token_address, amount)
        }
        ExecuteMsg::Withdraw { token_address, amount } => {
            execute::withdraw(deps, _env, info, token_address, amount)
        }
        ExecuteMsg::Borrow {
            borrow_token_address,
            amount,
            collateral_token_address,
        } => execute::borrow(
            deps,
            _env,
            info,
            borrow_token_address,
            amount,
            collateral_token_address,
        ),
        ExecuteMsg::Repay { token_address, amount } => {
            execute::repay(deps, _env, info, token_address, amount)
        }
        
        // ExecuteMsg::Liquidate {
        //     borrower,
        //     debt_token,
        //     collateral_token,
        // } => {
        //     execute::liquidate(deps, _env, info, borrower, debt_token, collateral_token)
        // }
        // ExecuteMsg::AddLiquidity {
        //     token1,
        //     token2,
        //     amount1,
        //     amount2,
        // } => execute::add_liquidity(deps, info, token1, token2, amount1, amount2),
        // ExecuteMsg::AddLiquidityByEscrow {
        //     token_1,
        //     token_2,
        //     amount_1,
        //     amount_2,
        //     slippage_tolerance,
        // } => execute::add_liquidity_by_escrow(
        //     deps,
        //     _env,
        //     info,
        //     token_1,
        //     token_2,
        //     amount_1,
        //     amount_2,
        //     slippage_tolerance,
        // ),
        // ExecuteMsg::CreateLendingPool { params } => {
        //     execute::create_lending_pool(deps, _env, info, params)
        // }
        // ExecuteMsg::UpdatePrice {
        //     token_address,
        //     new_price,
        // } => execute::update_price(deps, info, token_address, new_price),
    }
}

pub mod execute {
    use super::*;

    pub fn deposit(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
    ) ->  Result<Response, ContractError> {
        let mut account = ACCOUNT.load(deps.storage)?;
        let depositor = info.sender;
    
        // Check if depositor is the owner
        if account.owner != depositor {
            return Err(ContractError::Unauthorized{});
        }
    
        // Update user's deposit for the specific token
        let _current_balance = account.balance;

   
        // Update the total balance
        account.balance += amount;
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // If token is native, it's already sent with the transaction
        // If it's a cw20 token, we need to execute a transfer
        let _transfer_msg = if token_address == "native" {
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
            .add_attribute("balance", account.balance.to_string()))
    }

    pub fn withdraw(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let mut account = ACCOUNT.load(deps.storage)?;
        let withdrawer = info.sender;
    
        // Check if the withdrawer is the owner
        if account.owner != withdrawer {
            return Err(ContractError::Unauthorized{});
        }

        if account.balance < amount {
            return Err(ContractError::InsufficientFunds{});
        }
    
        

        account.balance -= amount;
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // Transfer tokens to user
        // let transfer_msg = if token_address == "native" {
        //     None
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
            .add_attribute("balance", account.balance.to_string()))
    }


    pub fn borrow(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        borrow_token_address: String,
        amount: Uint128,
        collateral_token_address: String,
    ) -> Result<Response, ContractError>  {
        const BORROW_RATIO : Decimal = Decimal::percent(80);
        let amount_usdt = amount;
        let amount_usdt_decimal = Decimal::from_atomics(amount_usdt, 0).unwrap();
        let collateral_usdt = amount_usdt_decimal / BORROW_RATIO;
        let collateral_native = collateral_usdt.clone();

        let mut account = ACCOUNT.load(deps.storage)?;
    

        let borrower = info.sender.clone();
        // Check if borrower is the owner
        if account.owner != borrower {
            return Err(ContractError::Unauthorized{});
        }
    
        // Check if the borrower has enough collateral
        let current_balance = account.balance;

       let collateral_usdt_int = collateral_usdt.to_uint_floor();

       let collateral_native_int = collateral_native.to_uint_floor() ;

        if current_balance < collateral_usdt_int {
            return Err(ContractError::InsufficientFunds{});
        }

        account.debt = account.debt + amount_usdt;
    
        // Update borrower's collateral and borrow
        ACCOUNT_STORAGE.deposits.save(deps.storage, collateral_token_address.clone(), &collateral_native_int)?;

        
  
    
        // Save updated account
        ACCOUNT.save(deps.storage, &account)?;
    
        // Transfer borrowed tokens to user
        // let transfer_msg = if borrow_token_address == "native" {
        //     ExecuteMsg::Send {
        //         to_address: borrower.to_string(),
        //         amount: vec![Coin {
        //             denom: "uarch".to_string(),
        //             amount,
        //         }],
        //     }
        // } else {
            // let transfer_msg = ExecuteMsg::Borrow {
            //     contract_addr: borrow_token_address.clone(),
            //     msg: to_json_binary(&Cw20ExecuteMsg::Transfer {
            //         recipient: borrower.to_string(),
            //         amount,
            //     })?,
            //     funds: vec![],
            // };
        // };
    
        Ok(Response::new()
            .add_attribute("action", "borrow")
            .add_attribute("borrower", borrower)
            .add_attribute("token", borrow_token_address)
            .add_attribute("debt", account.debt.to_string())
            .add_attribute("balance", account.balance.to_string())
         
            .add_attribute("amount", amount.to_string())
            .add_attribute("collateral_token", collateral_token_address)
            .add_attribute("collateral_amount", collateral_usdt.to_string()))
    }


    pub fn repay(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_address: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        
        let repayer = info.sender.clone();
    
        // Convert the repayment amount to USDT
        let amount_usdt = amount.clone();
        
        // Load the account
        let mut account = ACCOUNT.load(deps.storage)?;
    
        // Check if the repayer is the owner of the account
        if account.owner != repayer {
            return Err(ContractError::Unauthorized{});
        }
    
        // Load the borrowed amount for the token from the `borrows` map
        let borrowed_amount = ACCOUNT_STORAGE.borrows.may_load(deps.storage, token_address.clone())?;
    
        // Check if the borrow exists
        if borrowed_amount.is_none() {
            return Err(ContractError::TokenNotFound{});
        }
    
        // Extract the actual borrowed amount
        let mut token_amount = borrowed_amount.unwrap();
    
        // Ensure the repayment amount does not exceed the borrowed amount
        if token_amount < amount {
            return Err(ContractError::InsufficientFunds{});
        }
    
        // Reduce the borrowed amount and the debt
        token_amount -= amount;
        account.debt -= amount_usdt;
    
        // If the borrowed amount is fully repaid, remove the token entry from `borrows`
        if token_amount == Uint128::new(0) {
            ACCOUNT_STORAGE.borrows.remove(deps.storage, token_address.clone());
        } else {
            // Otherwise, save the updated borrowed amount
            ACCOUNT_STORAGE.borrows.save(deps.storage, token_address.clone(), &token_amount)?;
        }
    
        // Save the updated account
        ACCOUNT.save(deps.storage, &account)?;
        
        // Handle token transfer (if it's not native)
        let _transfer_msg = if token_address == "native" {
            None
        } else {
            Some(ExecuteMsg::Repay {
                token_address: token_address.clone(),
                amount: amount,
            })
        };
        
        Ok(Response::new()
            .add_attribute("action", "repay")
            .add_attribute("repayer", repayer)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount.to_string()))
    }
    
    
    // pub fn liquidate(
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     borrower: String,
    //     debt_token: String,
    //     collateral_token: String,
    // ) -> Result<Response, ContractError> {
    //     let mut account = ACCOUNT.load(deps.storage)?;
    //     let mut account_storage = ACCOUNT_STORAGE.load(deps.storage)?;
    //     let liquidator = info.sender;
    //     let borrower_addr = deps.api.addr_validate(&borrower)?;
    
    //     // Get the borrowed amount and collateral
    //     let borrowed_amount = account_storage.borrows.may_load(deps.storage,debt_token.clone())?.unwrap_or_default();

    //     let collateral_amount = account_storage.deposits.may_load(deps.storage,collateral_token.clone()).unwrap_or_default();
    
    //     // Update state
    //     account_storage.borrows.remove(deps.storage,debt_token.clone());
    //     account_storage.deposits.remove(deps.storage, collateral_token.clone());
    
    //     // Save updated account
    //     ACCOUNT.save(deps.storage, &account)?;
    
    //     // Transfer debt tokens from liquidator to contract
    //     // let debt_transfer_msg = ExecuteMsg::Execute {
    //     //     contract_addr: debt_token.clone(),
    //     //     msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
    //     //         owner: liquidator.to_string(),
    //     //         recipient: _env.contract.address.to_string(),
    //     //         amount: borrowed_amount,
    //     //     })?,
    //     //     funds: vec![],
    //     // };
    
    //     // Transfer collateral to liquidator
    //     let collateral_transfer_msg = ExecuteMsg::Liquidate {
    //         borrower: borrower_addr.to_string(),
    //         debt_token: debt_token.clone(),
    //         collateral_token: collateral_token.clone(),
    //     };
    
    //     Ok(Response::new()
    //         .add_attribute("action", "liquidate")
    //         .add_attribute("liquidator", liquidator)
    //         .add_attribute("borrower", borrower)
    //         .add_attribute("debt_token", debt_token)
    //         .add_attribute("debt_amount", borrowed_amount.to_string())
    //         .add_attribute("collateral_token", collateral_token)
    //         .add_attribute("collateral_amount", collateral_amount))
    // }

    // pub fn add_liquidity(
    //     deps: DepsMut,
    //     info: MessageInfo,
    //     token1: String,
    //     token2: String,
    //     amount1: Uint128,
    //     amount2: Uint128,
    // ) -> Result<Response, ContractError> {
    //     // Preparing the Euclid add_liquidity message
    //     let euclid_msg = ExecuteMsg::AddLiquidity {
    //         token1,
    //         token2,
    //         amount1,
    //         amount2,
    //     };
    
    //     // let wasm_msg = WasmMsg {
    //     //     contract_addr: "euclid_protocol_contract_address".to_string(), // Replace with actual Euclid contract address
    //     //     msg: to_json_binary(&euclid_msg)?,
    //     //     funds: vec![],
    //     // };
    
    //     Ok(Response::new()
    //         // .add_message(wasm_msg)
    //         .add_attribute("action", "add_liquidity")
    //         .add_attribute("from", info.sender)
    //         .add_attribute("amount1", amount1.to_string())
    //         .add_attribute("amount2", amount2.to_string()))
    // }


    // pub fn add_liquidity_by_escrow(
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     token_1: String,
    //     token_2: String,
    //     amount_1: Uint128,
    //     amount_2: Uint128,
    //     slippage_tolerance: u64,
    // ) -> StdResult<Response> {
    //     // Here we would typically interact with the Euclid SDK
    //     // For now, we'll just create a placeholder message
    //     // let msg = CosmosMsg::Wasm(WasmMsg::Execute {
    //     //     contract_addr: info.sender.to_string(),
    //     //     msg: to_json_binary(&"add_liquidity")?,
    //     //     funds: vec![],
    //     // });
    
    //     Ok(Response::new()
    //         // .add_message(msg)
    //         .add_attribute("action", "add_liquidity")
    //         .add_attribute("token_1", token_1)
    //         .add_attribute("token_2", token_2)
    //         .add_attribute("amount_1", amount_1.to_string())
    //         .add_attribute("amount_2", amount_2.to_string())
    //         .add_attribute("slippage_tolerance", slippage_tolerance.to_string()))
    // }

    // pub fn create_lending_pool(
    //     deps: DepsMut,
    //     env: Env,
    //     _info: MessageInfo,
    //     params: LendingPoolParams,
    // ) -> StdResult<Response> {
    //     // Load the state from storage and handle if it does not exist
    //     let state_bytes = deps.storage.get(b"state").ok_or(StdError::not_found("State"))?;
    
    //     // Convert Vec<u8> to Binary
    //     let state_binary = Binary(state_bytes);
    
    //     // Deserialize the state from Binary
    //     let mut state: State = from_binary(&state_binary)?;
    
    //     // Create the message to instantiate a new lending pool
    //     let msg = CosmosMsg::Wasm(WasmMsg::Instantiate {
    //         admin: Some(env.contract.address.to_string()),
    //         code_id: state.lending_pool_code_id,
    //         msg: to_json_binary(&params)?,
    //         funds: vec![],
    //         label: "whatever".to_string(),
    //     });
    
    //     // Return the response with the message included
    //     Ok(Response::new()
    //         .add_message(msg)
    //         .add_attribute("action", "create_lending_pool")
    //         .add_attribute("creator", _info.sender.to_string())) // Include any attributes you want to add
    // }

   

    pub fn convert_to_usdt(
        _deps: DepsMut,
        _info: MessageInfo,
        _token_address: String,
        amount: Uint128,
    ) -> StdResult<Uint128> {
        // Dummy conversion logic
        // For now, we'll just assume the conversion rate is 1:1 for simplicity.
        
        // You can add more logic later once integrated with an oracle
        let converted_amount = amount; // Dummy conversion
        
        Ok(converted_amount)  // Return the amount as the converted value
    }

    pub fn convert_to_token(
        _deps: DepsMut,
        _info: MessageInfo,
        _token_address: String,
        amount: Uint128,
    ) -> StdResult<Uint128> {
        // Dummy conversion logic
        // For now, we'll just assume the conversion rate is 1:1 for simplicity.
        
        // You can add more logic later once integrated with an oracle
        let converted_amount = amount; // Dummy conversion
        
        Ok(converted_amount)  // Return the amount as the converted value
    }

    // pub fn remove_liquidity(
    //     deps: DepsMut,
    //     info: MessageInfo,
    //     vlp_address: String,
    //     lp_allocation: Uint128,
    //     timeout: u64,
    //     cross_chain_addresses: Vec<String>,
    // ) -> StdResult<WasmMsg> {
    //     let rest_endpoint = "https://euclid-layer.rest/api"; // Example REST endpoint for Euclid Layer
    
    //     // Create REST call message for liquidity removal
    //     let msg = WasmMsg::Execute {
    //         contract_addr: rest_endpoint.to_string(),
    //         msg: to_json_binary(&{
    //             let rest_payload = serde_json::json!({
    //                 "vlp_address": vlp_address,
    //                 "lp_allocation": lp_allocation,
    //                 "sender": {
    //                     "address": info.sender.to_string(),
    //                     "chain_uid": "chain-uid-placeholder"
    //                 },
    //                 "timeout": timeout,
    //                 "cross_chain_addresses": cross_chain_addresses,
    //             });
    //             rest_payload
    //         })?,
    //         funds: vec![],
    //     };
    
    //     Ok(msg)
    // }

    // pub fn execute_swap(
    //     deps: DepsMut, 
    //     _env: Env, 
    //     info: MessageInfo, 
    //     amount_in: Uint128, 
    //     asset_in: String, 
    //     asset_out: String, 
    //     min_amount_out: Uint128,
    //     swaps: Vec<String>,
    //     cross_chain_addresses: Vec<String>,
    //     timeout: Option<u64>,
    // ) -> Result<Response, StdError> {
    //     // Check if the min_amount_out condition is met
    //     if amount_in < min_amount_out {
    //         return Err(StdError::generic_err("Slippage tolerance not met"));
    //     }
    
    //     // Create swap message
    //     // let msg = CosmosMsg::Bank(BankMsg::Send {
    //     //     to_address: cross_chain_addresses[0].clone(),
    //     //     amount: vec![Coin {
    //     //         denom: asset_in.clone(),
    //     //         amount: amount_in,
    //     //     }],
    //     // });
    
    //     Ok(Response::new()
    //         // .add_message(msg)
    //         .add_attribute("method", "execute_swap")
    //         .add_attribute("amount_in", amount_in)
    //         .add_attribute("asset_in", asset_in)
    //         .add_attribute("asset_out", asset_out)
    //     )
    // }

    // pub fn simulate_swap(
    //     deps: Deps, 
    //     asset_in: String, 
    //     amount_in: Uint128, 
    //     asset_out: String, 
    //     swaps: Vec<String>
    // ) -> StdResult<Binary> {
    //     let simulated_amount_out = Uint128::new(1000); // Hardcoded for simulation
    //     let swap = SimulatedSwap {
    //         amount_out: simulated_amount_out,
    //         asset_out: asset_out.clone(),
    //     };
    
    //     to_json_binary(&swap)
    // }
    

    // pub fn increment(deps: DepsMut) -> Result<Response, ContractError> {
    //     STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //         state.count += 1;
    //         Ok(state)
    //     })?;

    //     Ok(Response::new().add_attribute("action", "increment"))
    // }

    // pub fn reset(deps: DepsMut, info: MessageInfo, count: Uint128) -> Result<Response, ContractError> {
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


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};
    use crate::state::{Account, ACCOUNT, ACCOUNT_STORAGE};
    use crate::contract::{instantiate, execute};
    

    #[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();  // Mock the dependencies

    // Create a mock InstantiateMsg (modify if you have fields in InstantiateMsg)
    let msg = InstantiateMsg {};
    
    // Mock MessageInfo with sender "creator" and some initial funds
    let info = mock_info("creator", &coins(1000, "earth"));

    // Call the instantiate function and assert that it succeeds
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    
    // Check that no messages were sent in the response
    assert_eq!(0, res.messages.len());

    // Verify that attributes are set correctly
    assert_eq!(res.attributes[0], ("method", "instantiate"));
    assert_eq!(res.attributes[1], ("owner", "creator"));
    assert_eq!(res.attributes[2], ("id", "0"));  // The first account should have ID 0

    // Query the state to verify that the account was created correctly
    let account: Account = ACCOUNT.load(&deps.storage).unwrap();
    assert_eq!(account.id, Uint128::new(0));
    assert_eq!(account.balance, Uint128::new(0));
    assert_eq!(account.debt, Uint128::new(0));
    assert_eq!(account.owner, info.sender);
    // assert!(ACCOUNT_STORAGE.deposits(&deps.storage).is_empty());
    // assert!(ACCOUNT_STORAGE.borrows(&deps.storage).is_empty());

    // Verify that the ID counter was incremented
    let current_id = ID_COUNTER.load(&deps.storage).unwrap();
    assert_eq!(current_id, Uint128::new(1));
}


#[test]
fn test_execute_deposit() {
    let mut deps = mock_dependencies();
        let env = mock_env();
    
    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
    // Create an account
    let account = Account {
        id: Uint128::new(0),
        balance: Uint128::new(100),
        debt: Uint128::new(0),
        owner: info.sender.clone(),
        
    };

    if account.owner != info.sender {
        panic!("Unauthorized");
    }

    // let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    ACCOUNT.save(deps.as_mut().storage, &account).unwrap();

    let msg = ExecuteMsg::Deposit {
        token_address: "native".to_string(),
        amount: Uint128::new(50),
    };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Check response attributes
    assert_eq!(res.attributes.len(), 4);
    assert_eq!(res.attributes[0], ("action", "deposit"));
    assert_eq!(res.attributes[1], ("depositor", "owner"));
    assert_eq!(res.attributes[2], ("token", "native"));
    assert_eq!(res.attributes[3], ("balance", "150"));

    // Check that balance is updated
    let updated_account = ACCOUNT.load(&deps.storage).unwrap();
    assert_eq!(updated_account.balance, Uint128::new(150));
}


#[test]
fn test_execute_withdraw() {
    let mut deps = mock_dependencies();
        let env = mock_env();
    
    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create an account with sufficient balance
    let account = Account {
        id: Uint128::new(0),
        balance: Uint128::new(200),
        debt: Uint128::new(0),
        owner: info.sender.clone(),
    };

    if account.owner != info.sender {
        panic!("Unauthorized");
    }

    

    ACCOUNT.save(deps.as_mut().storage, &account).unwrap();

    let msg = ExecuteMsg::Withdraw {
        token_address: "native".to_string(),
        amount: Uint128::new(50),
    };

    

    // Call withdraw function
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Check response attributes
    assert_eq!(res.attributes.len(), 4);
    assert_eq!(res.attributes[0], ("action", "withdraw"));
    assert_eq!(res.attributes[1], ("withdrawer", "owner"));
    assert_eq!(res.attributes[2], ("token", "native"));
    assert_eq!(res.attributes[3], ("balance", "150"));

    // Check that balance is updated
    let updated_account = ACCOUNT.load(&deps.storage).unwrap();
    assert_eq!(updated_account.balance, Uint128::new(150));
}

#[test]
fn test_execute_borrow() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let amount = Uint128::new(100);  
    let borrow_token_address = "native".to_string();
    let mut_deps = deps.as_mut();

    let account = Account {
        id: Uint128::new(0),
        balance: Uint128::new(500),
        debt: Uint128::new(150), // Current debt
        owner: info.sender.clone(),
        
    };

    ACCOUNT.save(mut_deps.storage, &account).unwrap();


    let msg = ExecuteMsg::Borrow {
        borrow_token_address: borrow_token_address.clone(),
        amount: amount.clone(),
        collateral_token_address: "native_collateral".to_string(),
    };



    // Call withdraw function
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();



    // Check response attributes
    assert_eq!(res.attributes.len(), 8);
    assert_eq!(res.attributes[0], ("action", "borrow"));
    assert_eq!(res.attributes[1], ("borrower", "owner"));
    assert_eq!(res.attributes[2], ("token", "native"));
    assert_eq!(res.attributes[3], ("debt", "250"));

    assert_eq!(res.attributes[4], ("balance", "500"));

    assert_eq!(res.attributes[5], ("amount", "100"));

    assert_eq!(res.attributes[6], ("collateral_token", "native_collateral"));

    assert_eq!(res.attributes[7], ("collateral_amount", "125"));



    // // Check that balance is updated
    // let updated_account = ACCOUNT.load(&deps.storage).unwrap();
    // assert_eq!(updated_account.balance, 150.0);
}


#[test]
fn test_execute_repay() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {};
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    let amount = Uint128::new(50);  
    let token_address = "native".to_string();
    let mut_deps = deps.as_mut();

    let account = Account {
        id: Uint128::new(0),
        balance: Uint128::new(500),
        debt: Uint128::new(150), // Current debt
        owner: info.sender.clone(),
        
    };

    ACCOUNT.save(mut_deps.storage, &account).unwrap();


    let msg = ExecuteMsg::Repay {
        token_address: token_address.clone(),
        amount: amount.clone(),
        
    };

    ACCOUNT_STORAGE.borrows.save(mut_deps.storage,token_address.clone(),&amount);



    // Call withdraw function
    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();



    // Check response attributes
    assert_eq!(res.attributes.len(), 4);
    assert_eq!(res.attributes[0], ("action", "repay"));
    assert_eq!(res.attributes[1], ("repayer", "owner"));
    assert_eq!(res.attributes[2], ("token", "native"));

    assert_eq!(res.attributes[3], ("amount", "50"));


}

}
