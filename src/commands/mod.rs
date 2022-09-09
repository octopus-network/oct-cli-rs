use strum::{EnumDiscriminants, EnumIter, EnumMessage};

pub mod anchor_upgrade_command;

#[derive(Debug, Clone, EnumDiscriminants, interactive_clap_derive::InteractiveClap)]
#[strum_discriminants(derive(EnumMessage, EnumIter))]
#[interactive_clap(context = ())]
///Choose transaction action
pub enum TopLevelCommand {

    #[strum_discriminants(strum(message = "Anchor upgrade"))]
    AnchorUpgrade(self::anchor_upgrade_command::NetworkArgs)
}

impl TopLevelCommand {
    pub async fn process(self) -> crate::CliResult {
        match self {
            Self::AnchorUpgrade(anchor_upgrade_command) => anchor_upgrade_command.process().await
        }
    }
}
