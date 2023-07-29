// List of needed functions:
// 1. pub fn encode(path: PathBuf, chunk_type: ChunkType, message: &str) -> Result<()>
//  - this function will encode a message using the library code in png.rs
//  - we will need to read and write to files
// 2. pub fn encode_with_output_file(path: PathBuf, chunk_type: ChunkType, message: &str,
//    output_file: PathBuf) -> Result<()>
//    - does exactly the same thing as encode() but we can specify an output file
//    - rust doesn't have splats or multiple implementations, so that is what
//    I'm left with
// 3. pub fn decode(path: PathBuf, chunk_type: ChunkType) -> Result<String>
//      - decode a message and return it
// 4. pub fn remove(path: PathBuf, chunk_type: ChunkType) -> Result<()>
//  - removes a chunk with a given chunk_type
// 5. pub fn print(path: PathBuf) -> Result<String>
//  - prints all the chunks in a pretty way
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use std::fs;
use std::path::PathBuf;

pub fn encode(path: &PathBuf, chunk_type: &ChunkType, message: &str, output_file: &PathBuf) -> Result<()> {
    // Read the file from path and create a Png struct based on it's contents
    let image_data: Vec<u8> = fs::read(path)?;
    let mut image: Png = image_data.as_slice().try_into()?;

    // Create the chunk that we will append
    let chunk: Chunk = Chunk::new(chunk_type.clone(), message.as_bytes().to_vec());
    // Append the chunk (there might be fuckery with the IHDR and IEND chunks)
    // if the resulting image is not able to be opened, handle this here
    image.append_chunk(chunk);
    let modified_image: Vec<u8> = image.as_bytes();
    fs::write(output_file, modified_image)?;
    println!("Encoding successful!");
    Ok(())
}
pub fn decode(path: &PathBuf, chunk_type: &ChunkType) -> Result<()> {
    // for now, decode will just println! the result, is this optimal?
    let image_data: Vec<u8> = fs::read(path)?;
    let image: Png = image_data.as_slice().try_into()?;
    match image.chunk_by_type(&chunk_type.to_string()) {
        Some(x) => {
            println!("Message found: {}", x.data_as_string()?);
            Ok(())
        }
        None => Err("Specified chunk type not found".into()), 
    }
}
pub fn remove(path: &PathBuf, chunk_type: &ChunkType) -> Result<()> {
    // Read from file
    let image_data: Vec<u8> = fs::read(path)?;
    // Create png struct
    let mut image: Png = image_data.as_slice().try_into()?;
    // Remove chunk
    let removed_chunk = image.remove_chunk(&chunk_type.to_string())?;
    // Print message about removal
    println!("CHUNK: \"{}\" REMOVED", removed_chunk);

    // Write to file
    let modified_image: Vec<u8> = image.as_bytes();
    fs::write("out.png", modified_image)?;
    Ok(())
}
pub fn print(path: &PathBuf) -> Result<()> {
    let image_data: Vec<u8> = fs::read(path)?;
    let image: Png = image_data.as_slice().try_into()?;
    println!("{}", image);
    Ok(())
}
