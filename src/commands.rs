use crate::args;
use crate::png::{Png};
use crate::chunk_type::ChunkType; 
use std::str::FromStr; 
use crate::chunk::Chunk;
use std::fs; 


pub fn encode(args: &args::EncodeArgs)
{   
    let fp = args.file_path.clone(); 
    let chunkt_str = args.chunk_type.clone();
    let msg = args.message.clone(); 

    let content = fs::read(&fp).unwrap();

    let mut png = Png::try_from(&content[..]).unwrap(); 

    let chunkt = ChunkType::from_str(&chunkt_str).unwrap(); 
    let msg_b = msg.as_bytes().to_vec(); 
    let chunk = Chunk::new(chunkt, msg_b);

    png.append_chunk(chunk);

    let final_content = png.as_bytes(); 

    let _result = fs::write(&fp, final_content).unwrap(); 

    println!("We encode with : 
    file path : {:?}
    chunk type : {:?}
    message {:?}",
            fp, chunkt_str, msg);
}

pub fn decode(args: &args::DecodeArgs)
{
    let fp = args.file_path.clone(); 
    let chunkt_str = args.chunk_type.clone();
    let chunkt = ChunkType::from_str(&chunkt_str).unwrap(); 

    let content = fs::read(&fp).unwrap();
    let png = Png::try_from(&content[..]).unwrap(); 

    let chunkl = png.chunks(); 

    for chunk in chunkl.iter()
    {
        if chunk.chunk_type == chunkt
        {   
            println!("The hidden message is {:?}", chunk.data_as_string().unwrap());
            return; 
        }
    }

    println!("Hidden message not found");
}

pub fn remove(args: &args::RemoveArgs)
{
    let fp = args.file_path.clone(); 
    let chunkt_str = args.chunk_type.clone();
    let chunkt = ChunkType::from_str(&chunkt_str).unwrap(); 

    let content = fs::read(&fp).unwrap();
    let mut png = Png::try_from(&content[..]).unwrap(); 

    let result = png.remove_chunk(&String::from_utf8(chunkt.bytes().to_vec()).unwrap()).unwrap();

    let final_content = png.as_bytes(); 

    let _result = fs::write(&fp, final_content).unwrap(); 


    println!("chunk {:?} removec", result); 

}

pub fn print(args: &args::PrintArgs)
{
    let fp = args.file_path.clone(); 
    let chunkt = ChunkType::from_str("dEAD").unwrap(); 

    let content = fs::read(&fp).unwrap();
    let png = Png::try_from(&content[..]).unwrap(); 

    let chunkl = png.chunks(); 

    println!("The potential hidden messages are :\n"); 

    for chunk in chunkl.iter()
    {
        if chunk.chunk_type == chunkt
        {   
            println!("{:?}\n", chunk.data_as_string().unwrap());
        }
    }
    println!("End\n"); 
}