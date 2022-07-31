use clap::{Parser, Subcommand, AppSettings};

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// list and perform fs operations on vaults.
    VLT,
    /// fs operations for directories.
    DIR,
    /// switch directories with standard fs syntax.
    CDR,
    /// list and open notes from current vault's history.
    HST,
    /// open last accessed note in the current vault.
    LST,
    /// find directories and notes in the current vault.
    FND,
    /// list, create and delete memos or quick notes (independent of current vault).
    MEM,
    /// show this help message or help for given command.
    Help,
}