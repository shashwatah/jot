use clap::{ Parser, Subcommand };

#[derive(Parser, Debug)]
struct Args { 
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// list and perform fs operations on vaults.
    Vlt,
    /// list and perform fs operations on notes (in the current directory)
    Nts, 
    /// fs operations for directories. 
    Dir, 
    /// switch directories with standard fs syntax.
    Cdr,
    /// list and open notes from current vault's history.
    His,
    /// open last opened note in the current vault.
    Lst,
    /// find directories and notes in the current vault.
    Fnd,
    /// list, create and delete app wide memos or quick notes.
    Mem,
    /// show this help message.
    Help,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}