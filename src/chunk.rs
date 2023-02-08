//use std::{convert::TryFrom, str::pattern::CharArraySearcher};
use std::fmt;
use std::str::FromStr;
use std::str;
use crc::Crc; 

use crate::{Error, Result};
use crate::chunk_type::ChunkType;

pub struct Chunk{
    len : u32, 
    chunk_type : ChunkType,
    data : Vec<u8>,
    crc : u32,
}

impl Chunk{
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk
    {
        let len = data.len(); 
        let chunkt_bytes = chunk_type.bytes(); 
        let buff = [&data[1..len], &chunkt_bytes].concat();
        let crc = 
        let chunk = Chunk{
            len : 0, 
            chunk_type : ChunkType {sum :0},
            data : Vec::new(),
            crc : 0,
        };
        chunk
    }   
}

impl TryFrom<&[u8]> for Chunk{
    type Error = Error;
    fn try_from(vec : &[u8]) -> Result<Self>
    {   
        let mut len = 0; 
        let len_iter = vec[0..4].iter(); 
        for (i, byte) in len_iter.enumerate()
        {
            len += (byte << ((3 - i) * 8)) as  u32;
        }

        let list : [u8; 4] = vec[4..8].try_into().unwrap();
        let chunk_t = ChunkType::try_from(list).unwrap();

        let mut data = Vec::new();
        let data_end = (8 + len) as usize;
        let data_iter = vec[8..data_end].iter();
        for i in data_iter{
            data.push(*i);
        }

        let mut crc = 0; 
        let crc_iter = vec[0..4].iter(); 
        for (i, byte) in crc_iter.enumerate()
        {
            crc += (byte << ((3 - i) * 8)) as  u32;
        }

        let mut chunk = Chunk{
            len : len,
            chunk_type : chunk_t,
            data : data,
            crc :crc,
        };
        Ok(chunk)
    }
}

mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    // #[test]
    // fn testing_chunk()  {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
        
    //     Chunk::try_from(chunk_data.as_ref()).unwrap();
    // }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    // #[test]
    // fn test_chunk_length() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.length(), 42);
    // }

    // #[test]
    // fn test_chunk_type() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    // }

    // #[test]
    // fn test_chunk_string() {
    //     let chunk = testing_chunk();
    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");
    //     assert_eq!(chunk_string, expected_chunk_string);
    // }

    // #[test]
    // fn test_chunk_crc() {
    //     let chunk = testing_chunk();
    //     assert_eq!(chunk.crc(), 2882656334);
    // }

    // #[test]
    // fn test_valid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();

    //     let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

    //     let chunk_string = chunk.data_as_string().unwrap();
    //     let expected_chunk_string = String::from("This is where your secret message will be!");

    //     assert_eq!(chunk.length(), 42);
    //     assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //     assert_eq!(chunk_string, expected_chunk_string);
    //     assert_eq!(chunk.crc(), 2882656334);
    // }

    // #[test]
    // fn test_invalid_chunk_from_bytes() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656333;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();

    //     let chunk = Chunk::try_from(chunk_data.as_ref());

    //     assert!(chunk.is_err());
    // }

    // #[test]
    // pub fn test_chunk_trait_impls() {
    //     let data_length: u32 = 42;
    //     let chunk_type = "RuSt".as_bytes();
    //     let message_bytes = "This is where your secret message will be!".as_bytes();
    //     let crc: u32 = 2882656334;

    //     let chunk_data: Vec<u8> = data_length
    //         .to_be_bytes()
    //         .iter()
    //         .chain(chunk_type.iter())
    //         .chain(message_bytes.iter())
    //         .chain(crc.to_be_bytes().iter())
    //         .copied()
    //         .collect();
        
    //     let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
    //     let _chunk_string = format!("{}", chunk);
    // }
}