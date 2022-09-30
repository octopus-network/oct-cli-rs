use dialoguer::{theme::ColorfulTheme, Select};
use strum::{EnumMessage, IntoEnumIterator};

use crate::util::{is_executable, path_directories};

pub fn prompt_variant<T>(prompt: &str) -> T
where
    T: IntoEnumIterator + EnumMessage,
    T: Copy + Clone,
{
    let variants = T::iter().collect::<Vec<_>>();
    let actions = variants
        .iter()
        .map(|p| {
            p.get_message()
                .unwrap_or_else(|| "error[This entry does not have an option message!!]")
                .to_owned()
        })
        .collect::<Vec<_>>();

    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&actions)
        .default(0)
        .interact()
        .unwrap();

    variants[selected]
}

pub type CliResult = color_eyre::eyre::Result<()>;
pub fn try_external_subcommand_execution(error: clap::Error) -> CliResult {
    let (subcommand, args) = {
        let mut args = std::env::args().skip(1);
        let subcommand = args
            .next()
            .ok_or_else(|| color_eyre::eyre::eyre!("subcommand is not provided"))?;
        (subcommand, args.collect::<Vec<String>>())
    };
    let is_top_level_command_known = crate::commands::TopLevelCommandDiscriminants::iter()
        .map(|x| format!("{:?}", &x).to_lowercase())
        .find(|x| x == &subcommand)
        .is_some();
    if is_top_level_command_known {
        error.exit()
    }
    let subcommand_exe = format!("near-cli-{}{}", subcommand, std::env::consts::EXE_SUFFIX);

    let path = path_directories()
        .iter()
        .map(|dir| dir.join(&subcommand_exe))
        .find(|file| is_executable(file));

    let command = path.ok_or_else(|| {
        color_eyre::eyre::eyre!(
            "{} command or {} extension does not exist",
            subcommand,
            subcommand_exe
        )
    })?;

    let err = match cargo_util::ProcessBuilder::new(&command)
        .args(&args)
        .exec_replace()
    {
        Ok(()) => return Ok(()),
        Err(e) => e,
    };

    if let Some(perr) = err.downcast_ref::<cargo_util::ProcessError>() {
        if let Some(code) = perr.code {
            return Err(color_eyre::eyre::eyre!("perror occurred, code: {}", code));
        }
    }
    return Err(color_eyre::eyre::eyre!(err));
}
