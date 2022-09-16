use dialoguer::Input;
use itertools::Itertools;
use crate::CliResult;
use crate::commands::anchor_upgrade_command::select_server::server::ServerContext;
use crate::near::types::{NearEnv};
use crate::near::util::get_accounts_from_path;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::super::super::ServerContext)]
pub struct CustomDirectory {
    #[interactive_clap(long)]
    pub path: String,

    #[interactive_clap(named_arg)]
    pub upgrade:  super::upgrade::Upgrade
}

impl CustomDirectory {

    pub fn input_path(
        _context: &ServerContext
    )-> color_eyre::eyre::Result<String>{
        Ok(Input::new()
            .with_prompt("Enter custom directory path")
            .interact_text()?)
    }

    pub async fn process(self, connection_config: NearEnv) ->CliResult {
        let result = get_accounts_from_path(std::path::Path::new(self.path.as_str()))?;

        println!("Use these account to upgrade:");
        println!("[{}]", result.iter().map(|e|e.account_id.to_string()).join("\n"));

        self.upgrade.process(connection_config, result.into_iter().map(|e|e.into()).collect()).await
    }
}