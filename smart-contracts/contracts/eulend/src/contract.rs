#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, 
    Uint128, Decimal, StdError, Coin, BankMsg, Addr,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Account, ACCOUNTS, COLLATERAL};

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
        ExecuteMsg::Borrow { borrow_amount,collateral_denom , collateral_amount} => {
            execute::borrow(deps, env, info, borrow_amount, collateral_denom, collateral_amount)
        },
        ExecuteMsg::Repay { withdraw_denom, withdraw_amount } => {
            execute::repay(deps, env, info, withdraw_denom, withdraw_amount)
        },
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

    pub fn borrow(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        borrow_amount: Uint128,
        collateral_denom: String,
        collateral_amount: Uint128,
    ) -> Result<Response, ContractError> {
        
        let info_funds = info.funds
            .iter()
            .find(|coin| coin.denom == collateral_denom) ;
            
        
        // Load or create account
        let mut account = ACCOUNTS.may_load(deps.storage, &info.sender.to_string())?
            .unwrap();

        COLLATERAL.save(deps.storage, &collateral_denom, &collateral_amount)?;
            
        
        
        
        // Update borrowed amount
        account.borrowed_usdc += borrow_amount;
        
        // Save updated account
        ACCOUNTS.save(deps.storage, &info.sender.to_string(), &account)?;
        
        // Send USDC to borrower
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
            .add_attribute("collateral_denom", collateral_denom)
            .add_attribute("collateral_amount", collateral_amount)
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
            
        // Load account
        let mut account = ACCOUNTS.load(deps.storage, &info.sender.to_string())?;
        
        // Update borrowed amount
        account.borrowed_usdc = account.borrowed_usdc.checked_sub(usdc_repaid.amount)
            .map_err(|_| ContractError::NoRepayment {})?;
            
        // // Verify and update collateral
        let current_collateral = COLLATERAL.may_load(deps.storage, &withdraw_denom).unwrap(); 
            
            
        // // Calculate remaining collateral value after withdrawal
        let remaining_collateral = current_collateral ; 
            
            
        // Verify remaining collateral is sufficient for remaining loan
        let remaining_collateral_value = get_collateral_value(
            deps.as_ref(),
            &Coin {
                denom: withdraw_denom.clone(),
                amount: remaining_collateral.ok_or_else(|| ContractError::InsufficientFunds {})?,
            },
        )?;

        COLLATERAL.save(deps.storage, &withdraw_denom, &remaining_collateral_value)?;
        
        // if remaining_collateral_value < account.borrowed_usdc.checked_mul(COLLATERAL_RATIO)
        //     .ok_or(ContractError::MathError {})? {
        //     return Err(ContractError::InsufficientCollateral {});
        // }
        
        // Update remaining collateral
       
        
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
}

// Helper function to get collateral value (simplified)
fn get_collateral_value(deps: Deps, collateral: &Coin) -> Result<Uint128, ContractError> {
    // In practice, you would query an oracle here
    // This is a simplified version that assumes 1:1 value with USDC
    Ok(collateral.amount)
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     match msg {
//         QueryMsg::GetAccount { address } => {
//             to_json_binary(&query::get_account(deps, address)?)
//         },
//     }
// }

// pub mod query {
//     use super::*;

//     pub fn get_account(deps: Deps, address: String) -> StdResult<Account> {
//         ACCOUNTS.load(deps.storage, &address)
//     }
// }