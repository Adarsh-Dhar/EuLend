#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Decimal, StdError, Coin, WasmMsg};
use cw2::set_contract_version;
use cw_storage_plus::{Item};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg,InstantiateMsg, QueryMsg};
use crate::state::{Account, ACCOUNT_STORAGE,  ACCOUNTS, ORACLE_ADDRESS};

use std::time::Duration;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:backend";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const ID_COUNTER: Item<Uint128> = Item::new("id_counter");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Load the current ID counter, or initialize it to 0 if it doesn't exist
    
    let mut current_id = match ID_COUNTER.may_load(deps.storage)? {
        Some(id) => id,
        None => Uint128::new(0),
    };

    let account_key = current_id.to_string(); 


    // Create a new Account with an empty `deposits` and `borrows` Map
    let account = Account {
        id: current_id,
        balance: Uint128::new(0),
        debt: Uint128::new(0),
        owner: info.sender.clone(),
    };

    // Save the new account
    ACCOUNTS.save(deps.storage,&account_key, &account)?;

    

    // Increment the ID counter for the next account
    current_id += Uint128::new(1);
    ID_COUNTER.save(deps.storage, &current_id)?;

    // Set contract version (or any other state if necessary)
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Return a success response with attributes
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("id", account.id.to_string())
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit { token_address, amount, account_id } => {
            execute::deposit(deps, _env, info, token_address, amount, account_id)
        }
        ExecuteMsg::Withdraw { token_address, amount, account_id } => {
            execute::withdraw(deps, _env, info, token_address, amount, account_id)
        }
        ExecuteMsg::Borrow {
            borrow_token_address,
            amount,
            collateral_token_address,
            account_id
        } => execute::borrow(
            deps,
            _env,
            info,
            borrow_token_address,
            amount,
            collateral_token_address,
            account_id
        ),
        ExecuteMsg::Repay { token_address, amount, account_id } => {
            execute::repay(deps, _env, info, token_address, amount, account_id)
        }
        
       
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
        account_id : Uint128
    ) ->  Result<Response, ContractError> {
        let account_key = account_id.to_string(); 

        let mut account = ACCOUNTS.may_load(deps.storage, &account_key)?;
        let depositor = info.sender;
    
        // Check if depositor is the owner
        if account.clone().unwrap().owner != depositor {
            return Err(ContractError::Unauthorized{});
        }
    // Update the total balance
        let account = Account {
            id: account.clone().unwrap().id,
            balance: account.clone().unwrap().balance + amount,
            debt: account.clone().unwrap().debt,
            owner: account.clone().unwrap().owner,
        };
    
        // Save updated account
        ACCOUNTS.save(deps.storage,&account_key, &account)?;
    
        // If token is native, it's already sent with the transaction
        // If it's a cw20 token, we need to execute a transfer
        let _transfer_msg = if token_address == "native" {
            None
        } else {
            Some(ExecuteMsg::Deposit {
                token_address: token_address.clone(),
                amount: amount,
                account_id: account_id
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
        account_id : Uint128
    ) -> Result<Response, ContractError> {
        let account_key = account_id.to_string();
        let mut account = ACCOUNTS.may_load(deps.storage, &account_key)?;
        let withdrawer = info.sender;
    
        // Check if the withdrawer is the owner
        if account.clone().unwrap().owner != withdrawer {
            return Err(ContractError::Unauthorized{});
        }

        if account.clone().unwrap().balance < amount {
            return Err(ContractError::InsufficientFunds{});
        }
    
        let account = Account {
            id: account_id,
            balance: account.clone().unwrap().balance - amount,
            debt: account.clone().unwrap().debt,
            owner: account.clone().unwrap().owner,
        };
    
        // Save updated account
        ACCOUNTS.save(deps.storage,&account_key, &account)?;
    
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
        account_id : Uint128
    ) -> Result<Response, ContractError>  {
        let account_key = account_id.to_string();
        const BORROW_RATIO : Decimal = Decimal::percent(80);
        let amount_usdt = amount;
        let amount_usdt_decimal = Decimal::from_atomics(amount_usdt, 0).unwrap();
        let collateral_usdt = amount_usdt_decimal / BORROW_RATIO;
        let collateral_native = collateral_usdt.clone();

        let mut account = ACCOUNTS.may_load(deps.storage, &account_key)?;
    

        let borrower = info.sender.clone();
        // Check if borrower is the owner
        if account.clone().unwrap().owner != borrower {
            return Err(ContractError::Unauthorized{});
        }
    
        // Check if the borrower has enough collateral
        let current_balance = account.clone().unwrap().balance;

       let collateral_usdt_int = collateral_usdt.to_uint_floor();

       let collateral_native_int = collateral_native.to_uint_floor() ;

        if current_balance < collateral_usdt_int {
            return Err(ContractError::InsufficientFunds{});
        }

        let account = Account {
            id: account_id,
            balance: current_balance - collateral_usdt_int,
            debt: account.clone().unwrap().debt + amount_usdt,
            owner: account.clone().unwrap().owner,
        };
    
        // Update borrower's collateral and borrow
        ACCOUNT_STORAGE.deposits.save(deps.storage, collateral_token_address.clone(), &collateral_native_int)?;

        
  
    
        // Save updated account
        ACCOUNTS.save(deps.storage, &account_key, &account)?;
    
    
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
        account_id : Uint128
    ) -> Result<Response, ContractError> {
        let account_key = account_id.to_string();
        
        let repayer = info.sender.clone();
    
        // Convert the repayment amount to USDT
        let amount_usdt = amount.clone();
        
        // Load the account
        let mut account = ACCOUNTS.may_load(deps.storage, &account_key)?;
    
        // Check if the repayer is the owner of the account
        if account.clone().unwrap().owner != repayer {
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
        account.clone().unwrap().debt -= amount_usdt;

        let account = Account {
            id: account_id,
            balance: account.clone().unwrap().balance,
            debt: account.clone().unwrap().debt,
            owner: account.clone().unwrap().owner,
        };
    
        // If the borrowed amount is fully repaid, remove the token entry from `borrows`
        if token_amount == Uint128::new(0) {
            ACCOUNT_STORAGE.borrows.remove(deps.storage, token_address.clone());
        } else {
            // Otherwise, save the updated borrowed amount
            ACCOUNT_STORAGE.borrows.save(deps.storage, token_address.clone(), &token_amount)?;
        }
    
        // Save the updated account
        ACCOUNTS.save(deps.storage,&account_key, &account)?;
        
        // Handle token transfer (if it's not native)
        let _transfer_msg = if token_address == "native" {
            None
        } else {
            Some(ExecuteMsg::Repay {
                token_address: token_address.clone(),
                amount: amount,
                account_id: account_id
            })
        };
        
        Ok(Response::new()
            .add_attribute("action", "repay")
            .add_attribute("repayer", repayer)
            .add_attribute("token", token_address)
            .add_attribute("amount", amount.to_string()))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps : DepsMut,
    _env: Env,
    info : MessageInfo,
    msg : QueryMsg
) -> Result<Response, ContractError> {
    match msg {
        QueryMsg::GetOraclePrice { token_main, token_ref} => {
            query::get_oracle_price(deps, _env, info, token_main, token_ref)
        }
    }
}

pub mod query {
    use super::*;

    pub fn get_oracle_price(
        deps: DepsMut, 
        env: Env, 
        info: MessageInfo,
        token_main: String,
        token_ref : String
    ) -> Result<Response, ContractError> {
    
        let oracle_msg = to_json_binary(&GetReferenceData{
            symbol_pair : (token_main, token_ref)
        })?;
    
        let msg = WasmMsg::Execute {
            contract_addr: ORACLE_ADDRESS.to_string(),
            msg: oracle_msg,
            funds: vec![]
        };
    
        Ok(Response::new()
        .add_attribute("action", "get_oracle_price")
        .add_message(msg)
        )
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_info,
        mock_dependencies,
        mock_env,
        MockApi,
        MockQuerier,
        MockStorage,
        
    };
    use cosmwasm_std::{from_binary, coins, Timestamp, Addr, SystemError, WasmQuery,SystemResult, QuerierResult, OwnedDeps};
    use crate::state::{Account, ACCOUNTS, ACCOUNT_STORAGE};
    use crate::contract::{instantiate, execute};
  

    

    #[test]
fn test_instantiate() {
    let mut deps = mock_dependencies();  // Mock the dependencies

    // Create a mock InstantiateMsg (modify if you have fields in InstantiateMsg)
    let msg = InstantiateMsg {
        pyth_contract_addr: PYTH_CONTRACT_ADDR.to_string(),
        price_feed_id: PriceIdentifier::from_hex(PRICE_ID).unwrap(),
    };
    
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
    let account: Account = ACCOUNTS.load(deps.as_ref().storage, "0").unwrap();
    assert_eq!(account.clone().id, Uint128::new(0));
    assert_eq!(account.clone().balance, Uint128::new(0));
    assert_eq!(account.clone().debt, Uint128::new(0));
    assert_eq!(account.clone().owner, info.sender);
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
    let msg = InstantiateMsg {
        pyth_contract_addr: PYTH_CONTRACT_ADDR.to_string(),
        price_feed_id: PriceIdentifier::from_hex(PRICE_ID).unwrap(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
   
    
    // Create an account
    let account = Account {
        id: Uint128::new(0),
        balance: Uint128::new(100),
        debt: Uint128::new(0),
        owner: info.sender.clone(),
        
    };

    if account.clone().owner != info.sender {
        panic!("Unauthorized");
    }

    // let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    ACCOUNTS.save(deps.as_mut().storage, "0", &account).unwrap();

    let msg = ExecuteMsg::Deposit {
        token_address: "native".to_string(),
        amount: Uint128::new(50),
        account_id : Uint128::new(0),
    };

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Check response attributes
    assert_eq!(res.attributes.len(), 4);
    assert_eq!(res.attributes[0], ("action", "deposit"));
    assert_eq!(res.attributes[1], ("depositor", "owner"));
    assert_eq!(res.attributes[2], ("token", "native"));
    assert_eq!(res.attributes[3], ("balance", "150"));

    // Check that balance is updated
    let updated_account = ACCOUNTS.load(&deps.storage,"0").unwrap();
    assert_eq!(updated_account.clone().balance, Uint128::new(150));
}


#[test]
fn test_execute_withdraw() {
    let mut deps = mock_dependencies();
        let env = mock_env();
    
    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {
        pyth_contract_addr: PYTH_CONTRACT_ADDR.to_string(),
        price_feed_id: PriceIdentifier::from_hex(PRICE_ID).unwrap(),
    };
    let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Create an account with sufficient balance
    let account = Account {
        id: Uint128::new(0),
        balance: Uint128::new(200),
        debt: Uint128::new(0),
        owner: info.sender.clone(),
    };

    if account.clone().owner != info.sender {
        panic!("Unauthorized");
    }

    

    ACCOUNTS.save(deps.as_mut().storage,"0", &account).unwrap();

    let msg = ExecuteMsg::Withdraw {
        token_address: "native".to_string(),
        amount: Uint128::new(50),
        account_id : Uint128::new(0),
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
    let updated_account = ACCOUNTS.load(&deps.storage, "0").unwrap();
    assert_eq!(updated_account.clone().balance, Uint128::new(150));
}

#[test]
fn test_execute_borrow() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {
        pyth_contract_addr: PYTH_CONTRACT_ADDR.to_string(),
        price_feed_id: PriceIdentifier::from_hex(PRICE_ID).unwrap(),
    };
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

    ACCOUNTS.save(mut_deps.storage,"0", &account).unwrap();


    let msg = ExecuteMsg::Borrow {
        borrow_token_address: borrow_token_address.clone(),
        amount: amount.clone(),
        collateral_token_address: "native_collateral".to_string(),
        account_id : Uint128::new(0),
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



    // Check that balance is updated
    let updated_account = ACCOUNTS.load(&deps.storage, "0").unwrap();
    assert_eq!(updated_account.clone().balance, Uint128::new(375));
}


#[test]
fn test_execute_repay() {
    let mut deps = mock_dependencies();
    let env = mock_env();

    let info = mock_info("owner", &coins(1000, "earth"));
    let msg = InstantiateMsg {
        pyth_contract_addr: PYTH_CONTRACT_ADDR.to_string(),
        price_feed_id: PriceIdentifier::from_hex(PRICE_ID).unwrap(),
    };
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

    ACCOUNTS.save(mut_deps.storage,"0", &account).unwrap();


    let msg = ExecuteMsg::Repay {
        token_address: token_address.clone(),
        amount: amount.clone(),
        account_id : Uint128::new(0),
        
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
