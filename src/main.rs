mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    println!("Hello World");
    let clap_arg = args::Args::parse(); 


    println!("Hello {:?}", clap_arg.name);


    Ok(())
}

