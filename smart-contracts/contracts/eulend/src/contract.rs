#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Decimal, StdError, Coin};
use cw2::set_contract_version;
use cw_storage_plus::{Item};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg,InstantiateMsg, QueryMsg, FetchPriceResponse};
use crate::state::{Account, ACCOUNT,ACCOUNT_STORAGE, Oracle, ORACLE, ACCOUNTS};
use pyth_sdk_cw::{ get_update_fee,
get_valid_time_period,
    query_price_feed,
    PriceFeedResponse,};
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

    let oracle = Oracle {
        pyth_contract_addr: deps.api.addr_validate(msg.pyth_contract_addr.as_ref())?,
        price_feed_id: msg.price_feed_id,
    };
    ORACLE.save(deps.storage, &oracle)?;

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
        .add_attribute("price_id", format!("{}", msg.price_feed_id)))
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
    
   

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FetchPrice {} => to_json_binary(&query::query_fetch_price(deps, env)?),
        QueryMsg::FetchUpdateFee { vaas } => to_json_binary(&query::query_fetch_update_fee(deps, vaas)?),
        QueryMsg::FetchValidTimePeriod =>to_json_binary(&query::query_fetch_valid_time_period(deps)?),
    }
}

pub mod query {
    use super::*;

    

    pub fn query_fetch_price(deps: Deps, env: Env) -> StdResult<FetchPriceResponse> {
        let oracle = ORACLE.load(deps.storage)?;
    
       
        let price_feed_response: PriceFeedResponse =
            query_price_feed(&deps.querier, oracle.pyth_contract_addr, oracle.price_feed_id)?;
        let price_feed = price_feed_response.price_feed;
    
  
        // for recommendations.
        let current_price = price_feed
            .get_price_no_older_than(env.block.time.seconds() as i64, 60)
            .ok_or_else(|| StdError::not_found("Current price is not available"))?;
    
        // Get an exponentially-weighted moving average price and confidence interval.
        // The same notes about availability apply to this price.
        let ema_price = price_feed
            .get_ema_price_no_older_than(env.block.time.seconds() as i64, 60)
            .ok_or_else(|| StdError::not_found("EMA price is not available"))?;
    
        Ok(FetchPriceResponse {
            current_price,
            ema_price,
        })
    }
    
    pub fn query_fetch_update_fee(deps: Deps, vaas: Vec<Binary>) -> StdResult<Coin> {
        let oracle = ORACLE.load(deps.storage)?;
        let coin = get_update_fee(&deps.querier, oracle.pyth_contract_addr, vaas.as_slice())?;
        Ok(coin)
    }
    
    pub fn query_fetch_valid_time_period(deps: Deps) -> StdResult<Duration> {
        let oracle = ORACLE.load(deps.storage)?;
        let duration = get_valid_time_period(&deps.querier, oracle.pyth_contract_addr)?;
        Ok(duration)
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
    use crate::state::{Account, ACCOUNT, ACCOUNT_STORAGE};
    use crate::contract::{instantiate, execute};
    use pyth_sdk_cw::{
        testing::MockPyth,
        Price,
        PriceFeed,
        PriceIdentifier,
        UnixTimestamp,
    };


    const PYTH_CONTRACT_ADDR: &str = "pyth_contract_addr";
    // For real deployments, see list of price feed ids here https://pyth.network/developers/price-feed-ids
    const PRICE_ID: &str = "63f341689d98a12ef60a5cff1d7f85c70a9e17bf1575f0e7c0b2512d48b1c8b3";
    

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
    let account: Account = account.clone().unwrap().load(&deps.storage).unwrap();
    assert_eq!(account.clone().unwrap().id, Uint128::new(0));
    assert_eq!(account.clone().unwrap().balance, Uint128::new(0));
    assert_eq!(account.clone().unwrap().debt, Uint128::new(0));
    assert_eq!(account.clone().unwrap().owner, info.sender);
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

    if account.clone().unwrap().owner != info.sender {
        panic!("Unauthorized");
    }

    // let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    account.clone().unwrap().save(deps.as_mut().storage, &account).unwrap();

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
    let updated_account = account.clone().unwrap().load(&deps.storage).unwrap();
    assert_eq!(updated_account.clone().unwrap().balance, Uint128::new(150));
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

    if account.clone().unwrap().owner != info.sender {
        panic!("Unauthorized");
    }

    

    account.clone().unwrap().save(deps.as_mut().storage, &account).unwrap();

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
    let updated_account = account.clone().unwrap().load(&deps.storage).unwrap();
    assert_eq!(updated_account.clone().unwrap().balance, Uint128::new(150));
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

    account.clone().unwrap().save(mut_deps.storage, &account).unwrap();


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
    // let updated_account = account.clone().unwrap().load(&deps.storage).unwrap();
    // assert_eq!(updated_account.clone().unwrap().balance, 150.0);
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

    account.clone().unwrap().save(mut_deps.storage, &account).unwrap();


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

fn oracle_default_state() -> Oracle {
    Oracle {
        pyth_contract_addr: Addr::unchecked(PYTH_CONTRACT_ADDR),
        price_feed_id:      PriceIdentifier::from_hex(PRICE_ID).unwrap(),
    }
}

fn oracle_setup_test(
    oracle: &Oracle,
    mock_pyth: &MockPyth,
    block_timestamp: UnixTimestamp,
) -> (OwnedDeps<MockStorage, MockApi, MockQuerier>, Env) {
    let mut dependencies = mock_dependencies();

    let mock_pyth_copy = (*mock_pyth).clone();
    dependencies
        .querier
        .update_wasm(move |x| handle_wasm_query(&mock_pyth_copy, x));

    ORACLE.save(dependencies.as_mut().storage, oracle).unwrap();

    let mut env = mock_env();
    env.block.time = Timestamp::from_seconds(u64::try_from(block_timestamp).unwrap());

    (dependencies, env)
}

fn handle_wasm_query(pyth: &MockPyth, wasm_query: &WasmQuery) -> QuerierResult {
    match wasm_query {
        WasmQuery::Smart { contract_addr, msg } if *contract_addr == PYTH_CONTRACT_ADDR => {
            pyth.handle_wasm_query(msg)
        }
        WasmQuery::Smart { contract_addr, .. } => {
            SystemResult::Err(SystemError::NoSuchContract {
                addr: contract_addr.clone(),
            })
        }
        WasmQuery::Raw { contract_addr, .. } => {
            SystemResult::Err(SystemError::NoSuchContract {
                addr: contract_addr.clone(),
            })
        }
        WasmQuery::ContractInfo { contract_addr, .. } => {
            SystemResult::Err(SystemError::NoSuchContract {
                addr: contract_addr.clone(),
            })
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_get_price() {
    // Arbitrary unix timestamp to coordinate the price feed timestamp and the block time.
    let current_unix_time = 10_000_000;

    let mut mock_pyth = MockPyth::new(Duration::from_secs(60), Coin::new(1, "foo"), &[]);
    let price_feed = PriceFeed::new(
        PriceIdentifier::from_hex(PRICE_ID).unwrap(),
        Price {
            price:        100,
            conf:         10,
            expo:         -1,
            publish_time: current_unix_time,
        },
        Price {
            price:        200,
            conf:         20,
            expo:         -1,
            publish_time: current_unix_time,
        },
    );

    mock_pyth.add_feed(price_feed);

    let (deps, env) = oracle_setup_test(&oracle_default_state(), &mock_pyth, current_unix_time);

    let msg = QueryMsg::FetchPrice {};
    let result = query(deps.as_ref(), env, msg)
        .and_then(|binary| from_binary::<FetchPriceResponse>(&binary));

    assert_eq!(result.map(|r| r.current_price.price), Ok(100));
}

#[test]
fn test_query_fetch_valid_time_period() {
    // Arbitrary unix timestamp to coordinate the price feed timestamp and the block time.
    let current_unix_time = 10_000_000;

    let mock_pyth = MockPyth::new(Duration::from_secs(60), Coin::new(1, "foo"), &[]);
    let (deps, env) = oracle_setup_test(&oracle_default_state(), &mock_pyth, current_unix_time);

    let msg = QueryMsg::FetchValidTimePeriod {};
    let result =
        query(deps.as_ref(), env, msg).and_then(|binary| from_binary::<Duration>(&binary));

    assert_eq!(result.map(|r| r.as_secs()), Ok(60));
}

#[test]
fn test_query_fetch_update_fee() {
    // Arbitrary unix timestamp to coordinate the price feed timestamp and the block time.
    let current_unix_time = 10_000_000;

    let mock_pyth = MockPyth::new(Duration::from_secs(60), Coin::new(1, "foo"), &[]);
    let (deps, env) = oracle_setup_test(&oracle_default_state(), &mock_pyth, current_unix_time);

    let msg = QueryMsg::FetchUpdateFee {
        vaas: vec![Binary(vec![1, 2, 3])],
    };
    let result = query(deps.as_ref(), env, msg).and_then(|binary| from_binary::<Coin>(&binary));
    assert_eq!(result.map(|r| r.to_string()), Ok(String::from("1foo")))
}

}
}
