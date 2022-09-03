use crate::enums::{ConfigType, Item, VaultItem};
use clap::{AppSettings, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(global_setting(AppSettings::HidePossibleValuesInHelp))]
#[clap(global_setting(AppSettings::DontCollapseArgsInUsage))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::ColorNever))]
#[clap(before_help(
    r#"
                                    ________      _____ 
                                    ______(_)_______  /_
                                    _____  /_  __ \  __/
                                    ____  / / /_/ / /_  
                                    ___  /  \____/\__/  
                                    /___/ 
"#
))]
#[clap(help_template(
    "{before-help}{about-with-newline}{usage-heading}\n    {usage}\n\n{all-args}"
))]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// 📁 list and create vaults.
    #[clap(override_usage("jot vlt\n    jot vlt <vault name> <vault location>"))]
    Vl {
        /// show locations
        #[clap(parse(from_flag), short = 'l')]
        show_loc: bool,
        /// name for new vault.
        #[clap(value_parser, name = "vault name", requires = "vault location")]
        name: Option<String>,
        /// fs path to location of new vault.
        #[clap(value_parser, name = "vault location")]
        location: Option<PathBuf>,
    },
    /// 🚪 enter/switch to a vault.
    En {
        /// name of the vault to switch to.
        #[clap(value_parser, name = "vault name")]
        name: String,
    },
    /// 📝 create notes.
    #[clap(override_usage("jot nts\n    jot nts [note name]"))]
    Nt {
        /// name for new note (to be created in the current folder).
        #[clap(value_parser, name = "note name")]
        name: String,
    },
    /// 📖 open a note (from the current folder).
    Op {
        #[clap(value_parser, name = "note name")]
        name: String,
    },
    /// 📂 create folders.
    #[clap(override_usage("jot dir\n    jot dir [folder name]"))]
    Fd {
        /// name for new folder (to be created in the current folder).
        #[clap(value_parser, name = "folder name")]
        name: String,
    },
    /// 🔀 switch folders within current vault.
    Cd {
        /// path to location of folder to switch to (with current folder as root).
        #[clap(value_parser, name = "folder path")]
        path: PathBuf,
    },
    /// 🚮 delete a note/vault/folder.
    Rm {
        /// delete a note (nte) | vault (vlt) | folder (dir).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        /// name of item to be deleted.
        #[clap(value_parser, name = "name")]
        name: String,
    },
    /// 🔁 rename a note/vault/folder.
    Rn {
        /// rename a vault (vlt) | note (nte) | folder (dir).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        /// name of item to be renamed.
        #[clap(value_parser, name = "name")]
        name: String,
        /// new name of item.
        #[clap(value_parser, name = "new name")]
        new_name: String,
    },
    /// 🗃️ move a note/vault/folder.
    Mv {
        /// move a note (nte) | vault (vlt) | folder (dir).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        /// name of item to be moved.
        #[clap(value_parser, name = "name")]
        name: String,
        /// path to new location of item (current location as root in case of note or folder).
        #[clap(value_parser, name = "new location")]
        new_location: PathBuf,
    },
    /// 🗄️ move notes and folders from current vault to a different vault.
    Vm {
        /// move a note (nte) | folder (dir).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: VaultItem,
        /// name of item to be moved.
        #[clap(value_parser, name = "name")]
        name: String,
        /// name of vault to move the item to.
        #[clap(value_parser, name = "vault name")]
        vault_name: String,
    },
    /// 📃 List dir tree of current location
    Ls,
    /// Config
    Cf {
        #[clap(value_enum, value_parser, name = "config type")]
        config_type: ConfigType,
        #[clap(value_parser, name = "config value")]
        value: Option<String>,
    },
    /// 🆘 show this help message or help for given command.
    Help,
}
