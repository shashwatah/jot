use crate::args::Args;
use clap::Parser;

#[allow(dead_code)]
#[derive(Debug)]
pub struct App<'a> {
    name: &'a str,
    config: &'a str,
    args: Args,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App { name: "jot", config: "config", args: Args::parse() }        
    }
}