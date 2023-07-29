use crate::chunk_type::ChunkType;
use crate::Result;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt;
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}
impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error; // When referencing main we don't write crate::main

    fn try_from(value: &[u8]) -> Result<Self> {
        // The first four u8's are len
        // The next four u8's are type
        // The next len u8's are data
        // The next four u8's is CRC (because it is u32)
        // By that logic, value should be 4 + 4 + len + 4 = 12 + len long
        //println!("{:?}, {}", value,  value.len());
        let length: u32 = u32::from_be_bytes(value[0..4].try_into().unwrap());
        // For some reason, tests use BigEndian
        // from_be_bytes takes an array of bytes and turns them into big endian intiger
        let chunk_type = std::str::from_utf8(&value[4..8]).unwrap(); //Unwrap for now
        let chunk_type: ChunkType = chunk_type.parse().unwrap();
        let data: Vec<u8> = value[8..(length as usize + 8)].to_vec();
        let crc: u32 = u32::from_be_bytes(value[(length as usize + 8)..].try_into().unwrap());
        // For now we don't account for the fact, that array could be bad

        // Crc could be invalid, let's check for that
        if crc != Chunk::calculate_crc(&chunk_type, &data) {
            return Err("Invalid CRC provided".into());
        }

        let result = Chunk {
            length,
            chunk_type,
            data,
            crc,
        };
        Ok(result)
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // What is this fmt::Formattter?
        let message: String = format!(
            "Chunk Length: {}, Chunk Type: {}, ChunkData: {:?}, crc: {} \n",
            self.length(),
            self.chunk_type(),
            self.data(),
            self.crc()
        );
        write!(f, "{}", message)
    }
}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let checksum = Chunk::calculate_crc(&chunk_type, &data);
        Chunk {
            length: data.len() as u32,
            chunk_type, 
            data,
            crc: checksum,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        // If we return a reference, we DON'T clone
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(x) => Ok(x),
            Err(x) => Err(Box::new(x)),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.length().to_be_bytes().to_vec()); // Append the len
        result.append(&mut self.chunk_type().bytes().to_vec()); // Append chunk_type
        result.append(&mut self.data().to_vec()); // Append the chunk_data

        result.append(&mut self.crc.to_be_bytes().to_vec()); // Append the CRC
        result
    }

    pub fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let chunk_type_data = chunk_type.bytes();
        let crc_data: Vec<u8> = chunk_type_data
            .iter()
            .copied()
            .chain(data.iter().copied())
            .collect();
        let crc_obj = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc_obj.checksum(&crc_data)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
