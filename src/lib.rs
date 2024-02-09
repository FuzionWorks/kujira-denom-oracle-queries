//! This contract provides queries to get oracle prices for a kujira denomination
pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
