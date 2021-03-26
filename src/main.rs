use std::{io::Write, ops::Deref, process::Output};

use anyhow::{anyhow, Context, Result};
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "git-cz")]
struct Cli {}

const TYPES: [&str; 9] = [
    "feat", "fix", "refactor", "chore", "test", "docs", "style", "ci", "perf",
];
const max_header_length: usize = 50;
const min_message_length: usize = 3;
const max_message_length: usize = 64;

fn commit(message: String) -> Result<Output> {
    Command::new("git")
        .args(&["commit", "-m"])
        .arg(format!("\"{}\"", message))
        .output()
        .context("Failed to run git.")
}

fn main() -> Result<()> {
    Cli::from_args();

    let mut term = Term::stdout();
    let mut term_err = Term::stderr();

    let theme = ColorfulTheme::default();
    let commit_type = {
        let index = Select::with_theme(&theme)
            .items(&TYPES)
            .default(0)
            .interact()?;
        TYPES.get(index)
    }
    .ok_or(anyhow!("Need a type!"))?
    .deref();

    let message: String = Input::with_theme(&theme)
        .with_prompt(format!("{}:", commit_type))
        .validate_with(|input: &String| -> Result<(), String> {
            if input.len() < min_message_length {
                Err(format!(
                    "Message must be {} characters.",
                    min_message_length
                ))
            // "{}: {}"
            } else if input.len() + commit_type.len() + 2 > max_header_length {
                Err(format!(
                    "Message must be less than {} characters.",
                    max_header_length,
                ))
            } else {
                Ok(())
            }
        })
        .interact_text()?;

    let output = commit(format!("{}: {}", commit_type, message))?;

    term.write_all(&output.stdout)
        .context("Failed to write to stdout")?;
    term_err
        .write_all(&output.stderr)
        .context("Failed to write to stderr")?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to execute."))
    }
}
