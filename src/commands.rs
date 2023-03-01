use crate::args;

pub fn encode(args: &args::EncodeArgs)
{   
    let fp = args.file_path.clone(); 
    let chunkt = args.chunk_type.clone();
    let msg = args.message.clone(); 

    println!("We encode with : 
    file path : {:?}
    chunk type : {:?}
    message {:?}",
            fp, chunkt, msg);
}

pub fn decode(args: &args::DecodeArgs)
{
    let fp = args.file_path.clone(); 
    let chunkt = args.chunk_type.clone();

    println!("We decode with : 
    file path : {:?}
    chunk type : {:?}",
                fp, chunkt);
}

pub fn remove(args: &args::RemoveArgs)
{
    let fp = args.file_path.clone(); 
    let chunkt = args.chunk_type.clone();

    println!("We remove : 
    file path : {:?}
    chunk type : {:?}",
                fp, chunkt);
}

pub fn print(args: &args::PrintArgs)
{
    let fp = args.file_path.clone(); 

    println!("We print : 
    file path : {:?}",
                fp);
}