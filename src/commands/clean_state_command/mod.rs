pub mod select_env;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
pub struct CleanStateCommand {
    #[interactive_clap(subcommand)]
    select_env: self::select_env::SelectEnv,
}

impl CleanStateCommand {
    pub async fn process(self) -> crate::CliResult {
        self.select_env.process().await
    }
}