use crate::near::types::{NearAccountWithKey, NearBalance, NearEnv};
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_default_near_account_dir_path(connection_config: &NearEnv) -> PathBuf {
    let mut home_dir = dirs::home_dir().expect("Impossible to get your home dir!");
    home_dir.push(connection_config.account_dir_name());
    home_dir
}

pub fn get_accounts_from_path(path: &Path) -> color_eyre::eyre::Result<Vec<NearAccountWithKey>> {
    let mut accounts: Vec<NearAccountWithKey> = vec![];
    for file in fs::read_dir(path)
        .expect(format!("Failed to read dir use this path : {:?}\n", path).as_ref())
    {
        let entry = file.expect(format!("Failed to get file in path: {:?}.", path).as_ref());
        let file_type = entry
            .file_type()
            .expect(format!("Couldn't get file type for {:?}", entry).as_ref());
        if file_type.is_file() && entry.file_name().to_str().unwrap().ends_with(".json") {
            let account = NearAccountWithKey::from_file(&entry.path())
                .expect(format!("Failed to get signer from file.\n").as_ref());

            accounts.push(account)
        }
    }
    Ok(accounts)
}

pub fn print_transaction_status(
    transaction_info: near_primitives::views::FinalExecutionOutcomeView,
    network_connection_config: NearEnv,
) {
    match transaction_info.status {
        near_primitives::views::FinalExecutionStatus::NotStarted
        | near_primitives::views::FinalExecutionStatus::Started => unreachable!(),
        near_primitives::views::FinalExecutionStatus::Failure(tx_execution_error) => {
            print_transaction_error(tx_execution_error)
        }
        near_primitives::views::FinalExecutionStatus::SuccessValue(_) => {
            print_value_successful_transaction(transaction_info.clone())
        }
    };
    let transaction_explorer: url::Url = network_connection_config.transaction_explorer();
    println!("To see the transaction in the transaction explorer, please open this url in your browser:\n{path}{id}\n",
             id=transaction_info.transaction_outcome.id,
             path=transaction_explorer
    );
}

fn print_value_successful_transaction(
    transaction_info: near_primitives::views::FinalExecutionOutcomeView,
) {
    println!("Successful transaction");
    for action in transaction_info.transaction.actions {
        match action {
            near_primitives::views::ActionView::CreateAccount => {
                println!(
                    "New account <{}> has been successfully created.",
                    transaction_info.transaction.receiver_id,
                );
            }
            near_primitives::views::ActionView::DeployContract { code: _ } => {
                println!("Contract wasm code has been successfully deployed.",);
            }
            near_primitives::views::ActionView::FunctionCall {
                method_name,
                args: _,
                gas: _,
                deposit: _,
            } => {
                println!(
                    "The \"{}\" call to <{}> on behalf of <{}> succeeded.",
                    method_name,
                    transaction_info.transaction.receiver_id,
                    transaction_info.transaction.signer_id,
                );
            }
            near_primitives::views::ActionView::Transfer { deposit } => {
                println!(
                    "<{}> has transferred {} to <{}> successfully.",
                    transaction_info.transaction.signer_id,
                    NearBalance::from_yoctonear(deposit),
                    transaction_info.transaction.receiver_id,
                );
            }
            near_primitives::views::ActionView::Stake {
                stake,
                public_key: _,
            } => {
                println!(
                    "Validator <{}> has successfully staked {}.",
                    transaction_info.transaction.signer_id,
                    NearBalance::from_yoctonear(stake),
                );
            }
            near_primitives::views::ActionView::AddKey {
                public_key,
                access_key: _,
            } => {
                println!(
                    "Added access key = {} to {}.",
                    public_key, transaction_info.transaction.receiver_id,
                );
            }
            near_primitives::views::ActionView::DeleteKey { public_key } => {
                println!(
                    "Access key <{}> for account <{}> has been successfully deleted.",
                    public_key, transaction_info.transaction.signer_id,
                );
            }
            near_primitives::views::ActionView::DeleteAccount { beneficiary_id: _ } => {
                println!(
                    "Account <{}> has been successfully deleted.",
                    transaction_info.transaction.signer_id,
                );
            }
        }
    }
}

pub fn print_transaction_error(tx_execution_error: near_primitives::errors::TxExecutionError) {
    println!("Failed transaction");
    match tx_execution_error {
        near_primitives::errors::TxExecutionError::ActionError(action_error) => {
            print_action_error(action_error)
        }
        near_primitives::errors::TxExecutionError::InvalidTxError(invalid_tx_error) => {
            println!("{}", handler_invalid_tx_error(invalid_tx_error))
        }
    }
}

pub fn handler_invalid_tx_error(
    invalid_tx_error: near_primitives::errors::InvalidTxError,
) -> String {
    match invalid_tx_error {
        near_primitives::errors::InvalidTxError::InvalidAccessKeyError(invalid_access_key_error) => {
            match invalid_access_key_error {
                near_primitives::errors::InvalidAccessKeyError::AccessKeyNotFound{account_id, public_key} => {
                    format!("Error: Public key {} doesn't exist for the account <{}>.", public_key, account_id)
                },
                near_primitives::errors::InvalidAccessKeyError::ReceiverMismatch{tx_receiver, ak_receiver} => {
                    format!("Error: Transaction for <{}> doesn't match the access key for <{}>.", tx_receiver, ak_receiver)
                },
                near_primitives::errors::InvalidAccessKeyError::MethodNameMismatch{method_name} => {
                    format!("Error: Transaction method name <{}> isn't allowed by the access key.", method_name)
                },
                near_primitives::errors::InvalidAccessKeyError::RequiresFullAccess => {
                    format!("Error: Transaction requires a full permission access key.")
                },
                near_primitives::errors::InvalidAccessKeyError::NotEnoughAllowance{account_id, public_key, allowance, cost} => {
                    format!("Error: Access Key <{}> for account <{}> does not have enough allowance ({}) to cover transaction cost ({}).",
                            public_key,
                            account_id,
                            NearBalance::from_yoctonear(allowance),
                            NearBalance::from_yoctonear(cost)
                    )
                },
                near_primitives::errors::InvalidAccessKeyError::DepositWithFunctionCall => {
                    format!("Error: Having a deposit with a function call action is not allowed with a function call access key.")
                }
            }
        },
        near_primitives::errors::InvalidTxError::InvalidSignerId { signer_id } => {
            format!("Error: TX signer ID <{}> is not in a valid format or does not satisfy requirements\nSee \"near_runtime_utils::utils::is_valid_account_id\".", signer_id)
        },
        near_primitives::errors::InvalidTxError::SignerDoesNotExist { signer_id } => {
            format!("Error: TX signer ID <{}> is not found in the storage.", signer_id)
        },
        near_primitives::errors::InvalidTxError::InvalidNonce { tx_nonce, ak_nonce } => {
            format!("Error: Transaction nonce ({}) must be account[access_key].nonce ({}) + 1.", tx_nonce, ak_nonce)
        },
        near_primitives::errors::InvalidTxError::NonceTooLarge { tx_nonce, upper_bound } => {
            format!("Error: Transaction nonce ({}) is larger than the upper bound ({}) given by the block height.", tx_nonce, upper_bound)
        },
        near_primitives::errors::InvalidTxError::InvalidReceiverId { receiver_id } => {
            format!("Error: TX receiver ID ({}) is not in a valid format or does not satisfy requirements\nSee \"near_runtime_utils::is_valid_account_id\".", receiver_id)
        },
        near_primitives::errors::InvalidTxError::InvalidSignature => {
            format!("Error: TX signature is not valid")
        },
        near_primitives::errors::InvalidTxError::NotEnoughBalance {signer_id, balance, cost} => {
            format!("Error: Account <{}> does not have enough balance ({}) to cover TX cost ({}).",
                    signer_id,
                    NearBalance::from_yoctonear(balance),
                    NearBalance::from_yoctonear(cost)
            )
        },
        near_primitives::errors::InvalidTxError::LackBalanceForState {signer_id, amount} => {
            format!("Error: Signer account <{}> doesn't have enough balance ({}) after transaction.",
                    signer_id,
                    NearBalance::from_yoctonear(amount)
            )
        },
        near_primitives::errors::InvalidTxError::CostOverflow => {
            format!("Error: An integer overflow occurred during transaction cost estimation.")
        },
        near_primitives::errors::InvalidTxError::InvalidChain => {
            format!("Error: Transaction parent block hash doesn't belong to the current chain.")
        },
        near_primitives::errors::InvalidTxError::Expired => {
            format!("Error: Transaction has expired.")
        },
        near_primitives::errors::InvalidTxError::ActionsValidation(actions_validation_error) => {
            match actions_validation_error {
                near_primitives::errors::ActionsValidationError::DeleteActionMustBeFinal => {
                    format!("Error: The delete action must be the final action in transaction.")
                },
                near_primitives::errors::ActionsValidationError::TotalPrepaidGasExceeded {total_prepaid_gas, limit} => {
                    format!("Error: The total prepaid gas ({}) for all given actions exceeded the limit ({}).",
                            total_prepaid_gas,
                            limit
                    )
                },
                near_primitives::errors::ActionsValidationError::TotalNumberOfActionsExceeded {total_number_of_actions, limit} => {
                    format!("Error: The number of actions ({}) exceeded the given limit ({}).", total_number_of_actions, limit)
                },
                near_primitives::errors::ActionsValidationError::AddKeyMethodNamesNumberOfBytesExceeded {total_number_of_bytes, limit} => {
                    format!("Error: The total number of bytes ({}) of the method names exceeded the limit ({}) in a Add Key action.", total_number_of_bytes, limit)
                },
                near_primitives::errors::ActionsValidationError::AddKeyMethodNameLengthExceeded {length, limit} => {
                    format!("Error: The length ({}) of some method name exceeded the limit ({}) in a Add Key action.", length, limit)
                },
                near_primitives::errors::ActionsValidationError::IntegerOverflow => {
                    format!("Error: Integer overflow.")
                },
                near_primitives::errors::ActionsValidationError::InvalidAccountId {account_id} => {
                    format!("Error: Invalid account ID <{}>.", account_id)
                },
                near_primitives::errors::ActionsValidationError::ContractSizeExceeded {size, limit} => {
                    format!("Error: The size ({}) of the contract code exceeded the limit ({}) in a DeployContract action.", size, limit)
                },
                near_primitives::errors::ActionsValidationError::FunctionCallMethodNameLengthExceeded {length, limit} => {
                    format!("Error: The length ({}) of the method name exceeded the limit ({}) in a Function Call action.", length, limit)
                },
                near_primitives::errors::ActionsValidationError::FunctionCallArgumentsLengthExceeded {length, limit} => {
                    format!("Error: The length ({}) of the arguments exceeded the limit ({}) in a Function Call action.", length, limit)
                },
                near_primitives::errors::ActionsValidationError::UnsuitableStakingKey {public_key} => {
                    format!("Error: An attempt to stake with a public key <{}> that is not convertible to ristretto.", public_key)
                },
                near_primitives::errors::ActionsValidationError::FunctionCallZeroAttachedGas => {
                    format!("Error: The attached amount of gas in a FunctionCall action has to be a positive number.")
                }
            }
        },
        near_primitives::errors::InvalidTxError::TransactionSizeExceeded { size, limit } => {
            format!("Error: The size ({}) of serialized transaction exceeded the limit ({}).", size, limit)
        }
    }
}

pub fn print_action_error(action_error: near_primitives::errors::ActionError) {
    match action_error.kind {
        near_primitives::errors::ActionErrorKind::AccountAlreadyExists { account_id } => {
            println!("Error: Create Account action tries to create an account with account ID <{}> which already exists in the storage.", account_id)
        }
        near_primitives::errors::ActionErrorKind::AccountDoesNotExist { account_id } => {
            println!(
                "Error: TX receiver ID <{}> doesn't exist (but action is not \"Create Account\").",
                account_id
            )
        }
        near_primitives::errors::ActionErrorKind::CreateAccountOnlyByRegistrar {
            account_id: _,
            registrar_account_id: _,
            predecessor_id: _,
        } => {
            println!("Error: A top-level account ID can only be created by registrar.")
        }
        near_primitives::errors::ActionErrorKind::CreateAccountNotAllowed {
            account_id,
            predecessor_id,
        } => {
            println!("Error: A newly created account <{}> must be under a namespace of the creator account <{}>.", account_id, predecessor_id)
        }
        near_primitives::errors::ActionErrorKind::ActorNoPermission {
            account_id: _,
            actor_id: _,
        } => {
            println!("Error: Administrative actions can be proceed only if sender=receiver or the first TX action is a \"Create Account\" action.")
        }
        near_primitives::errors::ActionErrorKind::DeleteKeyDoesNotExist {
            account_id,
            public_key,
        } => {
            println!(
                "Error: Account <{}>  tries to remove an access key <{}> that doesn't exist.",
                account_id, public_key
            )
        }
        near_primitives::errors::ActionErrorKind::AddKeyAlreadyExists {
            account_id,
            public_key,
        } => {
            println!(
                "Error: Public key <{}> is already used for an existing account ID <{}>.",
                public_key, account_id
            )
        }
        near_primitives::errors::ActionErrorKind::DeleteAccountStaking { account_id } => {
            println!(
                "Error: Account <{}> is staking and can not be deleted",
                account_id
            )
        }
        near_primitives::errors::ActionErrorKind::LackBalanceForState { account_id, amount } => {
            println!("Error: Receipt action can't be completed, because the remaining balance will not be enough to cover storage.\nAn account which needs balance: <{}>\nBalance required to complete the action: <{}>",
                     account_id,
                     NearBalance::from_yoctonear(amount)
            )
        }
        near_primitives::errors::ActionErrorKind::TriesToUnstake { account_id } => {
            println!(
                "Error: Account <{}> is not yet staked, but tries to unstake.",
                account_id
            )
        }
        near_primitives::errors::ActionErrorKind::TriesToStake {
            account_id,
            stake,
            locked: _,
            balance,
        } => {
            println!(
                "Error: Account <{}> doesn't have enough balance ({}) to increase the stake ({}).",
                account_id,
                NearBalance::from_yoctonear(balance),
                NearBalance::from_yoctonear(stake)
            )
        }
        near_primitives::errors::ActionErrorKind::InsufficientStake {
            account_id: _,
            stake,
            minimum_stake,
        } => {
            println!(
                "Error: Insufficient stake {}.\nThe minimum rate must be {}.",
                NearBalance::from_yoctonear(stake),
                NearBalance::from_yoctonear(minimum_stake)
            )
        }
        near_primitives::errors::ActionErrorKind::FunctionCallError(function_call_error_ser) => {
            println!("Error: An error occurred during a `FunctionCall` Action, parameter is debug message.\n{:?}", function_call_error_ser)
        }
        near_primitives::errors::ActionErrorKind::NewReceiptValidationError(
            receipt_validation_error,
        ) => {
            println!("Error: Error occurs when a new `ActionReceipt` created by the `FunctionCall` action fails.\n{:?}", receipt_validation_error)
        }
        near_primitives::errors::ActionErrorKind::OnlyImplicitAccountCreationAllowed {
            account_id: _,
        } => {
            println!("Error: `CreateAccount` action is called on hex-characters account of length 64.\nSee implicit account creation NEP: https://github.com/nearprotocol/NEPs/pull/71")
        }
        near_primitives::errors::ActionErrorKind::DeleteAccountWithLargeState { account_id } => {
            println!(
                "Error: Delete account <{}> whose state is large is temporarily banned.",
                account_id
            )
        }
    }
}

#[test]
fn test() -> color_eyre::eyre::Result<()> {
    use std::str::FromStr;

    // let buf = get_default_near_account_dir_path(&NearEnv::Testnet);
    let buf = PathBuf::from_str("~/project/blockchian/octopus/oct-cli-rs").unwrap();
    let vec = get_accounts_from_path(buf.as_path())?;
    for signer in vec {
        println!("{}", serde_json::to_string(&signer).unwrap());
    }
    Ok(())
}
