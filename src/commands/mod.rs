use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod deploy_upgrade_command;
pub mod clean_state_command;
pub mod check_rewards;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///Choose transaction action
pub enum TopLevelCommand {

    #[strum_discriminants(strum(message = "Deploy or upgrade contract"))]
    DeployOrUpgrade(self::deploy_upgrade_command::DeployOrUpgrade),
    #[strum_discriminants(strum(message = "Clean state"))]
    CleanState(self::clean_state_command::CleanStateCommand),
    #[strum_discriminants(strum(message = "Check reward status"))]
    CheckReward(self::check_rewards::CheckReward),

}

impl TopLevelCommand {
    pub async fn process(self) -> crate::CliResult {
        match self {
            TopLevelCommand::DeployOrUpgrade(anchor_upgrade_command) => anchor_upgrade_command.process().await,

            TopLevelCommand::CleanState(clean_state_command) => clean_state_command.process().await,
            TopLevelCommand::CheckReward(check_rewards) => check_rewards.process().await
        }
    }
}
