pub mod anchor;
pub mod fungible_token;
pub mod registry;

use crate::near::rpc::client::Client;
use near_primitives::types::AccountId;

trait NearContract<'s> {
    fn get_account_id(&self) -> &AccountId;
    fn get_client(&self) -> &'s Client;
}
