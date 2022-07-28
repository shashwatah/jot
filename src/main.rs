use clap::{Parser};

/// Blazingly fast cli note management.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the vault to enter
    #[clap(short, long, value_parser)]
    enter_vault: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args)
}