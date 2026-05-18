mod config;
mod generate;

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};

use crate::config::Config;
use crate::generate::Generator;

#[derive(Parser)]
#[command(name = "cailoxo")]
#[command(about = "Generate adaptive native shell prompts")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Generate {
        #[arg(long, value_enum)]
        shell: Shell,

        #[arg(long, default_value = "cailoxo.toml")]
        config: PathBuf,

        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Shell {
    Zsh,
    Nu,
    Pwsh,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate {
            shell,
            config,
            output,
        } => {
            let config_text = fs::read_to_string(&config)
                .with_context(|| format!("failed to read {}", config.display()))?;
            let config = Config::parse(&config_text)
                .with_context(|| format!("failed to parse {}", config.display()))?;
            config.validate()?;

            let script = match shell {
                Shell::Zsh => Generator::new(&config).zsh(),
                Shell::Nu => Generator::new(&config).nu(),
                Shell::Pwsh => Generator::new(&config).pwsh(),
            }?;

            let output = output.unwrap_or_else(|| shell.default_output());
            if let Some(parent) = output.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            fs::write(&output, script)
                .with_context(|| format!("failed to write {}", output.display()))?;
        }
    }

    Ok(())
}

impl Shell {
    fn default_output(self) -> PathBuf {
        match self {
            Shell::Zsh => PathBuf::from("output/prompt.zsh"),
            Shell::Nu => PathBuf::from("output/prompt.nu"),
            Shell::Pwsh => PathBuf::from("output/prompt.ps1"),
        }
    }
}
