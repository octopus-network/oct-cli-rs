use near_crypto::InMemorySigner;
use near_primitives::{types::AccountId, views::FinalExecutionStatus};
use serde::{de::DeserializeOwned, Serialize};

use self::{constants::ONE_TERA_GAS, rpc::client::Client};

pub mod constants;
pub mod contracts;
pub mod gas;
pub mod rpc;
pub mod types;
pub mod util;

//
pub async fn call_contract_function_and_parse_result<T: DeserializeOwned + Serialize>(
    client: &Client,
    contract_id: &AccountId,
    signer: &InMemorySigner,
    method_name: String,
    args: String,
) -> color_eyre::eyre::Result<Option<T>, color_eyre::eyre::Report> {
    let function = format!("{}({})", method_name, args);
    let result = client
        .call(
            signer,
            contract_id,
            method_name,
            args.into_bytes(),
            ONE_TERA_GAS * 200,
            0,
        )
        .await
        .map_err(|err| {
            color_eyre::Report::msg(format!("Failed to call '{}'. {}", function, err))
        })?;
    match &result.status {
        FinalExecutionStatus::NotStarted => {
            return Err(color_eyre::Report::msg(format!(
                "Failed to call '{}'. Transaction not started.",
                function
            )))
        }
        FinalExecutionStatus::Started => {
            return Err(color_eyre::Report::msg(format!(
                "Failed to call '{}'. Transaction started, but no result.",
                function
            )))
        }
        FinalExecutionStatus::Failure(err) => {
            return Err(color_eyre::Report::msg(format!(
                "Failed to call '{}'. {}",
                function, err
            )))
        }
        FinalExecutionStatus::SuccessValue(bytes) => {
            if bytes.len() > 0 {
                let json = serde_json::de::from_slice::<T>(&bytes).unwrap();
                println!(
                    "Result of calling '{}': {}",
                    function,
                    serde_json::to_string(&json).unwrap()
                );
                return Ok(Some(json));
            } else {
                println!("Result of calling '{}': ''", function,);
                return Ok(None);
            }
        }
    }
}
