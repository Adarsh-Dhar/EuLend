#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, 
    Uint128, Decimal, StdError, Coin, BankMsg, Addr,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Account, ACCOUNTS, COLLATERAL, ESCROW, LIQUIDITY_PROVIDERS, LiquidityProvider, Collateral, Pool, POOLS, PoolUtilization};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:backend";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateAccount {} => execute::create_account(deps, info),
        ExecuteMsg::Borrow { borrow_amount,collateral_denom} => {
            execute::borrow(deps, env, info, borrow_amount, collateral_denom)
        },
        ExecuteMsg::Repay { withdraw_denom, withdraw_amount } => {
            execute::repay(deps, env, info, withdraw_denom, withdraw_amount)
        },
        ExecuteMsg::DeleteAccount {} => {
            execute::delete_account(deps, info)
        },
        ExecuteMsg::ProvideLiquidity {} => {
            execute::provide_liquidity(deps, env, info)
        }
        ExecuteMsg::ChangePoolUtilization { id } => {
            execute::change_pool_utilization(deps, id)
        }
    }
}

pub mod execute {
    use super::*;

    pub fn create_account(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        if ACCOUNTS.may_load(deps.storage, &info.sender.to_string())?.is_some() {
            return Err(ContractError::AccountExists {});
        }
    
        let account = Account {
            address: info.sender.to_string(),
            borrowed_usdc: Uint128::zero(),
        };
        
        ACCOUNTS.save(deps.storage, &info.sender.to_string(), &account)?;
    
        Ok(Response::new()
            .add_attribute("method", "create_account")
            .add_attribute("address", info.sender))
    }

    pub fn delete_account(
        deps: DepsMut,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let account = ACCOUNTS.load(deps.storage, &info.sender.to_string()).unwrap();
        if account.borrowed_usdc == Uint128::zero() && account.address.is_empty() {
            return Err(ContractError::AccountDoesNotExist {});
        }
        
        ACCOUNTS.remove(deps.storage, &info.sender.to_string());
        Ok(Response::new().add_attribute("method", "delete_account"))
    }

    //address = archway1h28ghlz7vm8e5j8mge3r9hkym9d6ldx9s9k094llgmer7h6snvjqujqxke

    pub fn borrow(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        borrow_amount: Uint128,
        collateral_denom: String,
    
        
    ) -> Result<Response, ContractError> {
        //getting funds from user and checking if collateral denom is correct
        let info_funds = info.funds
            .iter()
            .find(|coin| coin.denom == collateral_denom) ;
            
        //getting account with address from info
        let mut account = ACCOUNTS.may_load(deps.storage, &info.sender.to_string())?
            .unwrap();
        //checking if account do not exist
        if account.borrowed_usdc == Uint128::zero() && account.address.is_empty() {
            return Err(ContractError::AccountDoesNotExist {});
        }
        //entering values to the collateral struct
        let collateral = Collateral {
            address: info.sender.to_string(),
            token_denom : collateral_denom.clone(),
            amount : info_funds.unwrap().amount,
        };

        // Save collateral
        COLLATERAL.save(deps.storage, &collateral)?;
            
        // Get current escrow balance
        let mut escrow = ESCROW ;
        
        // Verify sufficient funds in escrow
        if escrow < borrow_amount {
            return Err(ContractError::InsufficientFunds {});
        }
        
        // Update escrow balance
        escrow = escrow.checked_sub(borrow_amount)
            .map_err(|_| ContractError::MathError {})?;
        
        
        // Update borrowed amount
        account.borrowed_usdc += borrow_amount;
        
        // Save updated account
        ACCOUNTS.save(deps.storage, &info.sender.to_string(), &account)?;
        
        // Send USDC to borrower from escrow
        let send_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: "usdc".to_string(),
                amount: borrow_amount,
            }],
        };
    
        Ok(Response::new()
            .add_message(send_msg)
            .add_attribute("method", "borrow")
            .add_attribute("borrower", info.sender)
            .add_attribute("collateral_denom", collateral_denom.clone())
            .add_attribute("collateral_amount", info_funds.unwrap().amount)
            .add_attribute("borrowed_amount", borrow_amount))
    }

    pub fn repay(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        withdraw_denom: String,
        withdraw_amount: Uint128,
    ) -> Result<Response, ContractError> {
        // Verify USDC was sent for repayment
        let usdc_repaid = info.funds
            .iter()
            .find(|coin| coin.denom == "usdc")
            .ok_or(ContractError::NoRepayment {})?;
            
        // Load account with address from info
        let mut account = ACCOUNTS.load(deps.storage, &info.sender.to_string())?;

        //checking if account do not exist  
        if account.borrowed_usdc == Uint128::zero() && account.address.is_empty() {
            return Err(ContractError::AccountDoesNotExist {});
        }
        
        // Update borrowed amount
        account.borrowed_usdc = account.borrowed_usdc - usdc_repaid.amount;
            
        // get current collateral value with address and denom
        let mut current_collateral = COLLATERAL.load(deps.storage)?;
        if current_collateral.address != info.sender.to_string() || current_collateral.token_denom != withdraw_denom {
            return Err(ContractError::TokenNotFound {});
        }

        //checking if withdraw amount is greater than current collateral
        if withdraw_amount > current_collateral.amount {
            return Err(ContractError::InsufficientFunds {});
        }

        //updating collateral amount
        current_collateral.amount = current_collateral.amount - withdraw_amount;
            
        COLLATERAL.save(deps.storage, &current_collateral)?;
    
        // Save updated account
        ACCOUNTS.save(deps.storage, &info.sender.to_string(), &account)?;
        
        // Return requested collateral
        let return_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin {
                denom: withdraw_denom.clone(),
                amount: withdraw_amount,
            }],
        };
        
        Ok(Response::new()
            .add_message(return_msg)
            .add_attribute("method", "repay")
            .add_attribute("repayer", info.sender)
            .add_attribute("usdc_repaid", usdc_repaid.amount)
            .add_attribute("collateral_withdrawn", withdraw_denom)
            .add_attribute("withdrawal_amount", withdraw_amount))
    }

    pub fn provide_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let liquidity_paid = info.funds
        .iter()
        .find(|coin| coin.denom == "usdc")
        .ok_or(ContractError::WrongToken {})?;
        

        // Create or update liquidity provider record
        let liquidity_provider = LiquidityProvider {
            address: info.sender.to_string(),
            liquidity_amount: liquidity_paid.amount,
        };
        LIQUIDITY_PROVIDERS.save(deps.storage, &liquidity_provider)?;


        Ok(Response::new()
            .add_attribute("method", "provide_liquidity")
            .add_attribute("provider", info.sender)
            .add_attribute("amount", liquidity_paid.amount))
    }

    pub fn change_pool_utilization(deps: DepsMut, id: Uint128) -> Result<Response, ContractError> {
        let mut pool = POOLS.load(deps.storage, &id.to_string())?;
        let total_liquidity = pool.total_liquidity;
        let total_borrowed = pool.total_borrowed;
    
        let utilization_ratio = total_borrowed / total_liquidity;
    
        if utilization_ratio > Uint128::from(90u128) {
            pool.utilization = PoolUtilization::High;
        } else if utilization_ratio > Uint128::from(50u128) {
            pool.utilization = PoolUtilization::Medium;
        } else {
            pool.utilization = PoolUtilization::Low;
        }
    
        POOLS.save(deps.storage, &id.to_string(), &pool)?;

        Ok(Response::new()
            .add_attribute("method", "change_pool_utilization")
            .add_attribute("pool_id", id.to_string())
            .add_attribute("pool", pool.utilization.to_string()))
    
    }
}



// Helper function to get collateral value (simplified)
fn get_collateral_value(deps: Deps, collateral: &Coin) -> Result<Uint128, ContractError> {
    // In practice, you would query an oracle here
    // This is a simplified version that assumes 1:1 value with USDC
    Ok(collateral.amount)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAccount { address } => {
            to_json_binary(&query::get_account(deps, address)?)
        },
        
    }
}

pub mod query {
    use super::*;

    pub fn get_account(deps: Deps, address: String) -> StdResult<Account> {
        let account = ACCOUNTS.load(deps.storage, &address)?;
        Ok(account)
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info,
        MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, Addr, OwnedDeps};
    use crate::msg::InstantiateMsg;

    const ADDR1: &str = "archway1t00mqwm46hmvkgj4ysyh0ykyjln3yw2fvt92wj";
    const ADDR2: &str = "archway1ehuphj3j9ml5stwan46syfv8rj9uw49mm7a5vy";

    fn setup() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(ADDR1, &[]);
        let env = mock_env();
        
        instantiate(deps.as_mut(), env, info, msg).unwrap();
        deps
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info(ADDR1, &[]);
        let env = mock_env();

        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(1, res.attributes.len());
        assert_eq!(("method", "instantiate"), res.attributes[0]);
    }

    #[test]
    fn test_create_account() {
        let mut deps = setup();
        let info = mock_info(ADDR1, &[]);

        // Test successful account creation
        let res = execute::create_account(deps.as_mut(), info.clone()).unwrap();
        assert_eq!(2, res.attributes.len());
        assert_eq!(("method", "create_account"), res.attributes[0]);
        assert_eq!(("address", ADDR1), res.attributes[1]);

        // Test duplicate account creation
        let err = execute::create_account(deps.as_mut(), info).unwrap_err();
        match err {
            ContractError::AccountExists {} => {}
            e => panic!("unexpected error: {:?}", e),

        }
    }



    #[test]
    fn test_borrow() {
        let mut deps = setup();
        
        // Create account first
        let info = mock_info(ADDR1, &[]);
        execute::create_account(deps.as_mut(), info.clone()).unwrap();

        // Test successful borrow
        let borrow_amount = Uint128::new(100);
        let collateral_amount = Uint128::new(200);
        let collateral_denom = "atom".to_string();
        
        let info = mock_info(
            ADDR1,
            &coins(200, "atom"), // Sending collateral
        );
        
        let res = execute::borrow(
            deps.as_mut(),
            mock_env(),
            info,
            borrow_amount,
            collateral_denom.clone(),
            collateral_amount,
        ).unwrap();

        // Verify response
        assert_eq!(5, res.attributes.len());
        assert_eq!(("method", "borrow"), res.attributes[0]);
        assert_eq!(("borrower", ADDR1), res.attributes[1]);
        assert_eq!(("collateral_denom", "atom"), res.attributes[2]);
        assert_eq!(("collateral_amount", "200"), res.attributes[3]);
        assert_eq!(("borrowed_amount", "100"), res.attributes[4]);

        // Verify bank message
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            cosmwasm_std::CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                assert_eq!(to_address, &ADDR1.to_string());
                assert_eq!(
                    amount,
                    &vec![Coin {
                        denom: "usdc".to_string(),
                        amount: borrow_amount,
                    }]
                );
            }
            _ => panic!("unexpected message type"),
        }
    }

    #[test]
    fn test_repay() {
        let mut deps = setup();
        
        // Setup: Create account and borrow first
        let info = mock_info(ADDR1, &[]);
        execute::create_account(deps.as_mut(), info.clone()).unwrap();
        
        let borrow_amount = Uint128::new(100);
        let collateral_amount = Uint128::new(200);
        let collateral_denom = "atom".to_string();
        
        let info = mock_info(
            ADDR1,
            &coins(200, "atom"),
        );
        
        execute::borrow(
            deps.as_mut(),
            mock_env(),
            info,
            borrow_amount,
            collateral_denom.clone(),
            collateral_amount,
        ).unwrap();

        // Test successful repayment
        let repay_info = mock_info(
            ADDR1,
            &coins(100, "usdc"), // Repaying USDC
        );
        
        let res = execute::repay(
            deps.as_mut(),
            mock_env(),
            repay_info,
            "atom".to_string(),
            Uint128::new(200), // Withdrawing all collateral
        ).unwrap();

        // Verify response
        assert_eq!(5, res.attributes.len());
        assert_eq!(("method", "repay"), res.attributes[0]);
        assert_eq!(("repayer", ADDR1), res.attributes[1]);
        assert_eq!(("usdc_repaid", "100"), res.attributes[2]);
        assert_eq!(("collateral_withdrawn", "atom"), res.attributes[3]);
        assert_eq!(("withdrawal_amount", "200"), res.attributes[4]);

        // Test repayment without sending USDC
        let no_funds_info = mock_info(ADDR1, &[]);
        let err = execute::repay(
            deps.as_mut(),
            mock_env(),
            no_funds_info,
            "atom".to_string(),
            Uint128::new(200),
        ).unwrap_err();
        
        match err {
            ContractError::NoRepayment {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }
}