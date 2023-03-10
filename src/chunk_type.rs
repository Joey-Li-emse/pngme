
//use std::{convert::TryFrom, str::pattern::CharArraySearcher};
use std::fmt;
use std::str::FromStr;
use std::str;

use crate::{Error, Result};


#[derive(Eq, Debug, PartialEq)]
pub struct ChunkType{
    pub sum : u32, 
}

#[allow(unused)]
impl ChunkType {

    const ANC_MASK : u32 = 1 << 29; 
    const PRI_MASK : u32 = 1 << 21; 
    const RES_MASK : u32 = 1 << 13; 
    const STC_MASK : u32 = 1 << 5; 
    const BYTE_MASK : u32 = 0xff; 

    pub fn bytes(&self) -> [u8; 4]{
        let mut array : [u8 ; 4] = [0; 4];
        let mut sum = self.sum; 
        for i in 0..4{
            array[3-i] = (sum & ChunkType::BYTE_MASK) as u8; 
            sum >>= 8;
        }
        array
    }

    pub fn is_valid(&self) -> bool{
        let mut sum = self.sum;
        for i in 0..4
        {
            let byte = (sum & ChunkType::BYTE_MASK) as u8;
            if  !byte.is_ascii_alphabetic()
                || (i == 1 && byte.is_ascii_lowercase())
            {
                return false
            } 
            sum >>= 8; 
        }
        true
    }

    pub fn is_critical(&self) -> bool{
        (self.sum & ChunkType::ANC_MASK) == 0
    }

    pub fn is_public(&self) -> bool{
        (self.sum & ChunkType::PRI_MASK) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool{
        (self.sum & ChunkType::RES_MASK) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool{
        (self.sum & ChunkType::STC_MASK) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes : [u8; 4]) -> Result<Self> 
    {   
        let mut chunk =  ChunkType{sum : 0};   
        for i in bytes
        {   
            chunk.sum <<= 8;
            chunk.sum += i as u32;
        }
        Ok(chunk)
    }
} 
impl FromStr for ChunkType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self>
    {   
        let mut chunk =  ChunkType{sum : 0};  
        for c in s.chars()
        {
            chunk.sum <<= 8;
            if !(c as u8).is_ascii_alphabetic()
            {
                return Err("Not Alphabetic".into()); 
            }
            chunk.sum += c as u32;
        }
        Ok(chunk)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sum = self.sum;
        let mut u8_list :[u8; 4] = [0;4];
        for i in 0..4
        {   
            let byte = (sum & ChunkType::BYTE_MASK) as u8;
            u8_list[3-i]= byte ;
            sum >>= 8;
        }
        let s = std::str::from_utf8(&u8_list[0..4]).expect("invalid utf-8 sequence");
        write!(f, "{}", s)
    }
}


#[allow(unused)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}



