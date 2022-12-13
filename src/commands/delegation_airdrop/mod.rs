pub mod select_env;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct DelegationAirdrop {
    #[interactive_clap(subcommand)]
    selected_env: self::select_env::SelectEnv,
}

impl DelegationAirdrop {
    pub async fn process(self) -> crate::CliResult {
        self.selected_env.process().await
    }
}
