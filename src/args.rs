use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args{
    /// Name of the person to greet
    pub name : Vec<String>,
}


