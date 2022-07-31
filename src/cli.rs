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
    /// list and switch vaults or perform fs operations on them.
    VLT {
        /// name for new vault.
        #[clap(value_parser, name="vault name", requires="vault path")]
        name: Option<String>,
        /// fs path of new vault.
        #[clap(value_parser, name="vault path")]
        path: Option<PathBuf>,
        #[clap(subcommand)]
        command: Option<VltCommand>,
    },
    /// list, open, rename, move, and delete notes.
    NTS {
        /// name for new note (to be created in current location).
        #[clap(value_parser, name="note name")]
        name: Option<String>,
        #[clap(subcommand)]
        command: Option<NtsCommand>,
    },
    /// display directory tree of current vault or perform fs operation on directories.
    DIR {
        /// name for new directory (to be created in current location).
        #[clap(value_parser, name="directory name")]
        name: Option<String>,
        #[clap(subcommand)]
        command: Option<DirCommand>,
    },
    /// switch directories within current vault.
    CDR {
        /// path of directory (with current location as root).
        #[clap(value_parser, name="directory path")]
        path: PathBuf,
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
        /// regex query string.
        #[clap(value_parser, name="query")]
        query: String,
        /// query files (fil) or directories (dir).
        #[clap(value_enum, value_parser, name="query type")]
        query_type: QueryType,
    },
    /// list, create and delete memos/quick notes (independent of current vault).
    MEM {
        /// content for new memo.
        #[clap(value_parser, name="content")]
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
    /// enter/switch to a vault.
    ENT { 
        #[clap(name="vault name")]
        name: String 
    },
    /// delete a vault.
    DEL {
        #[clap(name="vault name")]
        name: String
    },
    /// rename a vault.
    REN {
        #[clap(name="current name")]
        name: String,
        #[clap(name="new name")]
        new_name: String,
    },
    /// move vault to a new location in the fs.
    MOV {
        #[clap(name="vault name")]
        name: String,
        #[clap(name="new path")]
        new_path: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
enum NtsCommand {
    /// open a note with the editor defined in config.
    OPN {
        #[clap(name="note title")] 
        title: String
    },
    /// delete a note.
    DEL {
        #[clap(name="note title")] 
        title: String 
    },
    /// rename/retitle a note.
    REN { 
        #[clap(name="current title")]
        title: String,
        #[clap(name="new title")] 
        new_title: String 
    },
    /// move note to new location (with the current location as root).
    MOV {
        #[clap(name="note title")]
        title: String,
        #[clap(name="new location")]
        new_location: PathBuf,
    },
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
enum DirCommand {
    /// delete a directory.
    DEL { 
        #[clap(name="directory name")]
        name: String
    },
    /// rename a directory.
    REN { 
        #[clap(name="current name")]
        name: String, 
        #[clap(name="new name")]
        new_name: String
    },
    /// move directory to a new location within current vault (with current location as root).
    MOV {
        #[clap(name="directory name")]
        name: String,
        #[clap(name="new location")]
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
