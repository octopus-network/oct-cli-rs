use crate::near::rpc::client::Client;
use near_crypto::InMemorySigner;
use near_primitives::types::AccountId;
use near_primitives::views::FinalExecutionOutcomeView;

pub mod anchor;
pub mod registry;

trait NearContract<'s> {
    fn get_account_id(&self) -> &AccountId;
    fn get_client(&self) -> &'s Client;
}
