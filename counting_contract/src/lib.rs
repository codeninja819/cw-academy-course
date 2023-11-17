use cosmwasm_std::{
    Deps, DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point, to_json_binary
};
use cosmwasm_std::Binary;
 
mod contract;
pub mod msg;
 
#[entry_point]
pub fn instantiate(
	_deps: DepsMut,
	_env: Env,
	_info: MessageInfo,
	_msg: Empty,
) -> StdResult<Response> {
	Ok(Response::new())
}
 
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;
 
    match msg {
        Value {} => to_json_binary(&query::value()),
    }
}
