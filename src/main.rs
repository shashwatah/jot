mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
