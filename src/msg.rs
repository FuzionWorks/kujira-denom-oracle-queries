use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};
use kujira::Denom;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub oracle_config: Vec<OracleConfig>,
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        owner: Option<Addr>,
        oracle_config: Option<Vec<OracleConfig>>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(Coin)]
    Price { coin: Coin },
}

#[cw_serde]
pub struct OracleConfig {
    pub denom: Denom,
    pub oracle_denom: String,
    pub decimals: u8,
}

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub oracle_config: Vec<OracleConfig>,
}
