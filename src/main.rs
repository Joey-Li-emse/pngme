mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let clap_arg = args::Cli::parse(); 

    match &clap_arg.command{
        args::Commands::Encode(args) => commands::encode(args),
        args::Commands::Decode(args) => commands::decode(args),
        args::Commands::Remove(args) => commands::remove(args),
        args::Commands::Print(args) => commands::print(args),
    }


    Ok(())
}

