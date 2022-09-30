use std::collections::{HashMap, HashSet};
use color_eyre::owo_colors::OwoColorize;
use dialoguer::Input;
use itertools::Itertools;
use near_primitives::types::AccountId;
use prettytable::{Cell, ptable, row, Row, Table, table};
use crate::CliResult;
use crate::near::rpc::client::Client;
use crate::near::types::NearEnv;
use crate::oct::contracts::anchor::AnchorContract;
use crate::oct::contracts::anchor::types::{AnchorStatus, AppchainState};
use crate::oct::contracts::registry::RegistryContract;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct InputRegistryAccount {
    pub registry_account: String,
}

impl InputRegistryAccount {

    pub fn input_registry_account(
        _context: &()
    ) -> color_eyre::eyre::Result<String>  {
        Ok(Input::new()
            .with_prompt("What is the registry account?")
            .interact_text()?)
    }

    pub async fn process(
        self,
        connection_config: NearEnv,
        client: Client
    ) -> CliResult {

        let registry_contract = RegistryContract::new(self.registry_account.parse()?, &client );
        let appchain_ids = registry_contract.get_appchain_ids().await.map_err(|err| {
            color_eyre::Report::msg(format!("Failed to get appchain ids,contract_id:{}, error: {}", self.registry_account.clone(), err))
        }).unwrap();

        for appchain_id in appchain_ids {

            let anchor_contract_id: AccountId = format!("{}.{}", appchain_id, self.registry_account).parse()?;

            let anchor = AnchorContract {
                account_id: anchor_contract_id.clone(),
                client: &client
            };

            let appchain_state = registry_contract.get_appchain_status_of(appchain_id.clone()).await.map_err(|err| {
                color_eyre::Report::msg(format!("Failed to get anchor({}) state, error: {}",anchor_contract_id, err))
            } )?.appchain_state;

            if !(matches!(appchain_state, AppchainState::Active)) {
                println!("The state of {} is {:?}, skip check!\n", anchor_contract_id.blue(), appchain_state.yellow());
                continue;
            }

            let anchor_status = anchor.get_anchor_status().await.map_err(|err| {
                color_eyre::Report::msg(format!("Failed to get anchor({}) status, error: {}",anchor_contract_id, err))
            })?;
            let era_now = anchor_status.index_range_of_validator_set_history.end_index;

            let setting = anchor.get_protocol_settings().await.map_err(|err| {
                color_eyre::Report::msg(format!("Failed to get protocol settings, error: {}", err))
            })?;

            let start_check_era = if (setting.maximum_allowed_unprofitable_era_count as u64)>era_now {0} else {era_now-(setting.maximum_allowed_unprofitable_era_count as u64)};

            let mut era_with_unprofitable_validator_ids = HashMap::new();
            let mut unprofitable_account_ids = HashSet::new();

            for era in start_check_era..(era_now+1) {

                let info = anchor.get_validator_set_info_of(era).await.map_err(|err| {
                    color_eyre::Report::msg(format!("Failed to get validator set info, error: {}", err))
                })?.expect(format!("Error: Get empty validator set info of era({})", era).as_str());

                info.unprofitable_validator_ids.iter().for_each(|e|  {unprofitable_account_ids.insert(e.clone());} );

                era_with_unprofitable_validator_ids.insert(era, info.unprofitable_validator_ids);

            }
            print_auto_unbound_info_table(anchor_contract_id, unprofitable_account_ids, era_with_unprofitable_validator_ids);
        }

        Ok(())
    }
}

fn print_auto_unbound_info_table(
    anchor_contract_id: AccountId,
    unprofitable_account_ids: HashSet<AccountId>,
    era_with_unprofitable_validator_ids: HashMap<u64, Vec<AccountId>>
) {
    let mut table = Table::new();
    let eras = era_with_unprofitable_validator_ids.iter().map(|(era,_ )| *era ).sorted().collect_vec();

    assert!(eras.len()>0, "Failed to print_auto_unbound_info_table, the era len should > 0.");

    if unprofitable_account_ids.is_empty() {
        // head.add_cell(Cell::new(format!("No unprofitable validator in allowed time range.").green().to_string().as_str()));
        table.add_row(row![format!("No unprofitable validator from era_{} to era_{}.", eras[0], eras[eras.len()-1]).green()]);
    } else {

        let mut head = row!["validators\\eras"];

        eras.iter().for_each(|era| {head.add_cell(Cell::new(era.to_string().as_str()))});
        table.add_row(head);
        for account_id in unprofitable_account_ids {
            let mut row = row![account_id];
            for era in &eras {
                let is_unprofitable = era_with_unprofitable_validator_ids
                    .get(era)
                    .expect("Failed to get user's information unprofitable")
                    .contains(&account_id);
                let cell_content = if is_unprofitable {format!("×").red().to_string()}
                    else {format!("√").green().to_string()};
                row.add_cell(Cell::new(cell_content.as_str()));
            }
            table.add_row(row);
        }
    }

    let title = Row::new(vec![Cell::new(format!("{} unprofitable_validator_ids table", anchor_contract_id.blue()).as_str()).style_spec("H1")]);
    table.set_titles(title);

    table.printstd();
}

struct AutoBondInfo {
    anchor_contract_name: AccountId,
    current_era: u64,
}

#[test]
fn test() {
    let s = row![];
    let mut table = table!(["1","2","3"]);
    table.add_row(Row::new(vec![Cell::new(format!("No unprofitable validator in allowed time range.").green().to_string().as_str())]));
    table.printstd();

    let state = AppchainState::Active;
    println!("{}", matches!(state, AppchainState::Active));
    println!("{}", matches!(state, AppchainState::Frozen));
}