use clap::{
    Args,
    Parser,
    Subcommand
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli{
    /// Operation on png file
    #[command(subcommand)]
    pub command : Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands{
    /// Encode the file
    Encode(EncodeArgs),
    /// Decode the file
    Decode(DecodeArgs),
    /// Remove the chunk
    Remove(RemoveArgs),
    /// Print the file
    Print(PrintArgs), 
}

#[derive(Args, Debug)]
pub struct EncodeArgs{
    /// File path for the png file 
    pub file_path : String, 
    /// Chunk type 
    pub chunk_type : String,
    /// Message to encode
    pub message : String, 
}

#[derive(Args, Debug)]
pub struct DecodeArgs{
    /// File path for the png file 
    pub file_path : String, 
    /// Chunk type 
    pub chunk_type : String,
}

#[derive(Args, Debug)]
pub struct RemoveArgs{
    /// File path for the png file 
    pub file_path : String, 
    /// Chunk type 
    pub chunk_type : String,
}

#[derive(Args, Debug)]
pub struct PrintArgs{
    /// File path for the png file 
    pub file_path : String,
}



