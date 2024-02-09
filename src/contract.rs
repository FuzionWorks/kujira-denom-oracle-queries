use std::ops::{Div, Mul};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;
use kujira::{KujiraMsg, KujiraQuerier, KujiraQuery};

use crate::msg::Config;
use crate::state::CONFIG;
use crate::{ContractError, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

const CONTRACT_NAME: &str = "fuzion/kujira-denom-oracle-prices";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn migrate(deps: DepsMut<KujiraQuery>, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut<KujiraQuery>,
    _: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config {
        owner: msg.owner,
        oracle_config: msg.oracle_config,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<KujiraQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<KujiraMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::UpdateConfig {
            owner,
            oracle_config,
        } => {
            ensure!(info.sender == config.owner, ContractError::Unauthorized {});
            if let Some(owner) = owner {
                config.owner = owner;
            }
            if let Some(oracle_config) = oracle_config {
                config.oracle_config = oracle_config;
            }
            CONFIG.save(deps.storage, &config)?;
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<KujiraQuery>, _: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    Ok(match msg {
        QueryMsg::Config {} => to_json_binary(&config),
        QueryMsg::Price { coin } => {
            let oracle_config = config
                .oracle_config
                .iter()
                .find(|x| x.denom.to_string() == coin.denom);

            let value = if let Some(oracle_config) = oracle_config {
                let q = KujiraQuerier::new(&deps.querier);
                let res = q.query_exchange_rate(oracle_config.oracle_denom.to_string())?;
                let price = res.normalize(oracle_config.decimals);
                price * coin.amount
            } else {
                return Err(ContractError::InvalidDenom(coin.denom));
            };
            to_json_binary(&value)
        }
    }?)
}
