use crate::chunk_type::ChunkType;

pub struct Chunk{
    len : u32, 
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc : u32,
}

