#![allow(unused_variables)]
use std::str::FromStr;
#[derive(PartialEq, Eq, Debug, Clone)] // Derive Debug not specified in requirements,
                                       //might cause problems
pub struct ChunkType {
    // Each chunk has a type that can be represented as a 4 character string
    // for now I will use just String, although maybe Vec<u8> will be better
    // or some kind of byte string, I don't know
    // I could even set the type to [char; 4], or [u8; 4]
    chunk_type: String,
}
#[derive(Debug, Clone)]
pub struct ParseChunkTypeError;
impl std::fmt::Display for ParseChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "ParseChunkTypeError!")
    }
}
impl std::error::Error for ParseChunkTypeError {}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
            .as_bytes()
            .try_into()
            .expect("Conversion of ChunkType to bytes failed")
    }
    fn is_valid(&self) -> bool {
        // Check if reserved bit is not set
        // Check if length of chunk is 4
        if self.chunk_type.len() != 4 {
            return false;
        }
        if !self.is_reserved_bit_valid() {
            return false;
        }
        true
    }
    fn is_critical(&self) -> bool {
        // If bit 5 of first byte is '0' return True
        // Bytes are counted right to left, starting with 0 (like indexes in array)
        // A byte with bit 5 set is '00100000'
        let data = self.bytes();
        data[0] & (1 << 5) == 0
    }
    fn is_public(&self) -> bool {
        // If bit 5 of second byte is '0' return True
        let data = self.bytes();
        data[1] & (1 << 5) == 0
    }
    fn is_reserved_bit_valid(&self) -> bool {
        // If bit 5 of third byte is '0' return True
        let data = self.bytes();
        data[2] & (1 << 5) == 0
    }
    fn is_safe_to_copy(&self) -> bool {
        // If bit 5 of fourth byte is '1' return True
        let data = self.bytes();
        data[3] & (1 << 5) != 0
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error; // I need to access the Error type in main.rs
                               // Currently not sure how to do that
                               // Why is this 'crate' highlighted red?
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match std::str::from_utf8(&value) {
            Ok(value) => Ok(ChunkType {
                chunk_type: value.to_string(),
            }),
            Err(e) => Err(Box::new(e)),
        }
    }
}

impl FromStr for ChunkType {
    type Err = ParseChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 { return Err(ParseChunkTypeError); } // Why can't I write self::Err ??
        let new = ChunkType {
            chunk_type: s.to_string(),
        };

        for &val in s.as_bytes() {
            // This is really ugly, how to write this correctly in rust?
            if val < 65 || (val > 90 && val < 97) || val > 122 {
                return Err(ParseChunkTypeError);
            }
        }
        /* we can create chunks with not correct reserved bit, why?
        if new.is_valid() {
            return Ok(new);
        }
        else {
            return Err(Box::new(ParseChunkTypeError)); // What error to put here?
        }
        */
        Ok(new)
    }
}
impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.chunk_type)
    }
}

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
