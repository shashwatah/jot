use clap::{AppSettings, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// list and perform fs operations on vaults.
    VLT {
        /// name for the vault to be created.
        #[clap(value_parser)]
        vault_name: Option<String>,
        /// location for the vault to be created.
        #[clap(value_parser)]
        location: Option<PathBuf>,
        #[clap(subcommand)]
        command: Option<VltCommand>,
    },
    /// operations for notes.
    NTS {
        /// name for the note to be created.
        #[clap(value_parser)]
        note_name: Option<String>,
        /// location for the note to be created.
        #[clap(value_parser)]
        location: Option<PathBuf>,
        #[clap(subcommand)]
        command: Option<NtsCommand>,
    },
    /// fs operations for directories.
    DIR {
        /// name for the directory to be created.
        #[clap(value_parser)]
        dir_name: Option<String>,
        #[clap(subcommand)]
        command: Option<DirCommand>,
    },
    /// switch directories with standard fs syntax.
    CDR {
        /// location of the directory to switch to.
        #[clap(value_parser)]
        location: PathBuf,
    },
    /// list and open notes from current vault's history.
    HST {
        #[clap(subcommand)]
        command: Option<HstCommand>,
        // #[clap(short, action)]
        // open: bool
    },
    /// open last accessed note in the current vault.
    LST,
    /// find directories and notes in the current vault.
    FND {
        /// query directories or files
        #[clap(value_parser)]
        query: String,
        #[clap(value_enum, value_parser)]
        query_type: QueryType,
    },
    /// list, create and delete memos or quick notes (independent of current vault).
    MEM {
        /// memo content.
        #[clap(value_parser)]
        content: Option<String>,
        #[clap(subcommand)]
        command: Option<MemCommand>,
        // #[clap(short, action)]
        // del: bool
    },
    /// show this help message or help for given command.
    Help,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
enum VltCommand {
    /// enter a vault.
    ENT { vault_name: String },
    /// delete a vault.
    DEL { vault_name: String },
    /// rename a vault.
    REN {
        vault_name: String,
        new_name: String,
    },
    /// move vault to new location in the fs.
    MOV {
        vault_name: String,
        new_location: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
enum NtsCommand {
    /// enter a note.
    OPN { note_name: String },
    /// delete a note.
    DEL { note_name: String },
    /// rename a note.
    REN { note_name: String, new_name: String },
    /// move note to new location.
    MOV {
        note_name: String,
        new_location: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
enum DirCommand {
    /// delete a directory.
    DEL { dir_name: String },
    /// rename a directory.
    REN { dir_name: String, new_name: String },
    /// move directory to a new location.
    MOV {
        dir_name: String,
        new_location: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
enum HstCommand {
    /// open a note from history.
    OPN,
}

#[derive(ValueEnum, Clone, Debug)]
enum QueryType {
    FIL,
    DIR,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
enum MemCommand {
    /// choose which memo to delete.
    DEL,
}
