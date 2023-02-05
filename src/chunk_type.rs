
//use std::{convert::TryFrom, str::pattern::CharArraySearcher};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[allow(unused_variables)]
const ANC : u8 = 0; 
const PRI : u8 = 1;
const RES : u8 = 2; 
const STC : u8 = 3;  

const FIFTH_MASK : u8 = 1 << 4;
const ANC_MASK : u32 = 1 << 4; 
const PRI_MASK : u32 = 1 << 8; 
const RES_MASK : u32 = 1 << 12; 
const STC_MASK : u32 = 1 << 16; 
const BYTE_MASK : u32 = 0xff; 

struct ChunkType{
    sum : u32, 
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4]{
        let mut array : [u8 ; 4] = [0; 4];
        let mut sum = self.sum; 
        for i in 0..3{
            sum = sum >> (4*i);
            array[i] = (sum & BYTE_MASK) as u8; 
        }
        array
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes : [u8; 4]) -> Result<Self> 
    {   
        let mut i : u8;
        let mut chunk =  ChunkType{sum : 0};   
        for i in bytes
        {
            chunk.sum += i as u32;
            chunk.sum = chunk.sum << 8;  
        }
        Ok(chunk)
    }
} 




#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    // pub fn test_chunk_type_from_bytes() {
    //     let expected = [82, 117, 83, 116];
    //     let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

    //     assert_eq!(expected, actual.bytes());
    // }

    pub fn test_chunk_type_from_bytes() {
        let expected = 0x74537552;
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.sum);
    }

    // #[test]
    // pub fn test_chunk_type_from_str() {
    //     let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
    //     let actual = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(expected, actual);
    // }

    // #[test]
    // pub fn test_chunk_type_is_critical() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_critical() {
    //     let chunk = ChunkType::from_str("ruSt").unwrap();
    //     assert!(!chunk.is_critical());
    // }

    // #[test]
    // pub fn test_chunk_type_is_public() {
    //     let chunk = ChunkType::from_str("RUSt").unwrap();
    //     assert!(chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_not_public() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(!chunk.is_public());
    // }

    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_reserved_bit_valid());
    // }

    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_invalid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_reserved_bit_valid());
    // }

    // #[test]
    // pub fn test_chunk_type_is_safe_to_copy() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_chunk_type_is_unsafe_to_copy() {
    //     let chunk = ChunkType::from_str("RuST").unwrap();
    //     assert!(!chunk.is_safe_to_copy());
    // }

    // #[test]
    // pub fn test_valid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_valid());
    // }

    // #[test]
    // pub fn test_invalid_chunk_is_valid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_valid());

    //     let chunk = ChunkType::from_str("Ru1t");
    //     assert!(chunk.is_err());
    // }

    // #[test]
    // pub fn test_chunk_type_string() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(&chunk.to_string(), "RuSt");
    // }

    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}



