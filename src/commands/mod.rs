use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod deploy_upgrade_command;
pub mod clean_state_command;
pub mod check_unprofitable_validator;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///Choose transaction action
pub enum TopLevelCommand {

    #[strum_discriminants(strum(message = "Deploy or upgrade contract"))]
    DeployOrUpgrade(self::deploy_upgrade_command::DeployOrUpgrade),
    #[strum_discriminants(strum(message = "Clean state"))]
    CleanState(self::clean_state_command::CleanStateCommand),
    #[strum_discriminants(strum(message = "Check unprofitable validator"))]
    CheckUnprofitableValidator(self::check_unprofitable_validator::CheckUnprofitableValidator),

}

impl TopLevelCommand {
    pub async fn process(self) -> crate::CliResult {
        match self {
            TopLevelCommand::DeployOrUpgrade(anchor_upgrade_command) => anchor_upgrade_command.process().await,

            TopLevelCommand::CleanState(clean_state_command) => clean_state_command.process().await,
            TopLevelCommand::CheckUnprofitableValidator(check_unprofitable_validator) => check_unprofitable_validator.process().await
        }
    }
}
