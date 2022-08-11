use clap::{AppSettings, Parser, Subcommand, ValueEnum};

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
    pub subcommand: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    /// 📝 list and create notes.
    #[clap(override_usage("jot nts\n    jot nts [note name]"))]
    NTE {
        /// name for new note (to be created in the current folder).
        #[clap(value_parser, name = "note name")]
        name: Option<String>,
    },
    /// 📖 open a note (from the current folder).
    OPN {
        #[clap(value_parser, name = "note name")]
        name: String,
    },
    /// 📁 list and create vaults.
    #[clap(override_usage("jot vlt \n    jot vlt <vault name> <vault location>"))]
    VLT {
        /// name for new vault.
        #[clap(value_parser, name = "vault name", requires = "vault location")]
        name: Option<String>,
        /// fs path to location of new vault.
        #[clap(value_parser, name = "vault location")]
        location: Option<String>,
    },
    /// 🚪 enter/switch to a vault.
    ENT {
        /// name of the vault to switch to.
        #[clap(value_parser, name = "vault name")]
        name: String,
    },
    /// 📂 create folders and display dir tree of the current vault.
    #[clap(override_usage("jot dir\n    jot dir [folder name]"))]
    DIR {
        /// name for new folder (to be created in the current folder).
        #[clap(value_parser, name = "folder name")]
        name: Option<String>,
    },
    /// 🔀 switch folders within current vault.
    CDR {
        /// path to location of folder to switch to (with current folder as root).
        #[clap(value_parser, name = "folder location")]
        location: String,
    },
    /// 🗒️ list and open notes from current vault's history.
    #[clap(override_usage("jot hst\n    jot hst [SUBCOMMAND]"))]
    HST,
    /// ⏮️ open last accessed note in the current vault.
    LST,
    /// 🔍 find folders and notes in the current vault.
    FND {
        /// find notes (nte) or folders (dir).
        #[clap(value_enum, value_parser, name = "query type")]
        query_type: VaultItem,
        /// query string.
        #[clap(value_parser, name = "query")]
        query: String,
    },
    /// 📒 list, create and delete memos/quick notes (independent of current vault).
    #[clap(override_usage("jot mem\n    jot mem [content]\n    jot mem <SUBCOMMAND>"))]
    MEM {
        /// content for new memo.
        #[clap(value_parser, name = "content")]
        content: Option<String>,
        #[clap(subcommand)]
        subcommand: Option<MemSubCommand>,
    },
    /// 🔁 rename a note/vault/folder.
    REN {
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
    /// 🚮 delete a note/vault/folder.
    DEL {
        /// delete a note (nte) | vault (vlt) | folder (dir).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        /// name of item to be deleted.
        #[clap(value_parser, name = "name")]
        name: String,
    },
    /// 🗃️ move a note/vault/folder.
    MOV {
        /// move a note (nte) | vault (vlt) | folder (dir).
        #[clap(value_enum, value_parser, name = "item type")]
        item_type: Item,
        /// name of item to be moved.
        #[clap(value_parser, name = "name")]
        name: String,
        /// path to new location of item (current location as root in case of note or folder).
        #[clap(value_parser, name = "new location")]
        new_location: String,
    },
    /// 🗄️ move notes and folders from current vault to a different vault.
    MVV {
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
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Item {
    VLT,
    NTE,
    DIR,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum VaultItem {
    NTE,
    DIR,
}

#[derive(Subcommand, Debug)]
pub enum HstSubCommand {
    /// 📖 open a note from history.
    OPN,
    /// 🆘 show this help message or help for given command.
    Help,
}

#[derive(Subcommand, Debug)]
#[clap(args_conflicts_with_subcommands = true)]
pub enum MemSubCommand {
    /// 🚮 choose which memo to delete.
    DEL,
    /// 🆘 show this help message or help for given command.
    Help,
}
