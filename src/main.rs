#![allow(dead_code)]
#![allow(unused_variables)]

use crate::common::{try_external_subcommand_execution, CliResult};
use clap::Clap;
use serde::{Deserialize, Serialize};
use shell_words;
use util::*;

mod commands;
mod common;
mod near;
mod oct;
mod util;

#[derive(Debug, Clone, interactive_clap_derive::InteractiveClap)]
#[interactive_clap(context = ())]
struct Args {
    #[interactive_clap(subcommand)]
    top_level_command: self::commands::TopLevelCommand,
}

impl Args {
    async fn process(self) -> CliResult {
        self.top_level_command.process().await
    }
}

fn main() -> CliResult {
    color_eyre::install()?;

    let cli = match CliArgs::try_parse() {
        Ok(cli) => cli,
        Err(error) => {
            if matches!(error.kind, clap::ErrorKind::UnknownArgument) {
                return try_external_subcommand_execution(error);
            }
            error.exit();
        }
    };

    // if let Some(self::commands::CliTopLevelCommand::GenerateShellCompletions(subcommand)) =
    //     cli.top_level_command
    // {
    //     subcommand.process();
    //     return Ok(());
    // }

    let args = Args::from_cli(Some(cli), ())?;

    let completed_cli = CliArgs::from(args.clone());

    let process_result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(args.process());

    println!(
        "Your console command(you can execute this command directly next time) is:\n{} {}",
        std::env::args().next().as_deref().unwrap_or("./near_cli"),
        shell_words::join(&completed_cli.to_cli_args())
    );

    process_result
}
