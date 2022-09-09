use itertools::Itertools;
use crate::CliResult;
use crate::near::types::{NearEnv};
use crate::near::util::{get_accounts_from_path, get_default_near_account_dir_path};

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = super::super::super::ServerContext)]
pub struct DefaultDirectory {
    #[interactive_clap(named_arg)]
    pub upgrade:  super::upgrade::Upgrade
}

impl DefaultDirectory {
    pub async fn process(self, connection_config: NearEnv) ->CliResult {
        let buf = get_default_near_account_dir_path(&connection_config);
        let result = get_accounts_from_path(buf.as_path())?;

        println!("Use these account to upgrade:");
        println!("[{}]", result.iter().map(|e|e.account_id.to_string()).join("\n"));
        self.upgrade.process(connection_config, result.into_iter().map(|e|e.into()).collect()).await
    }
}