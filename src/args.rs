// There seems to be some fuckery
// In the impl of FromStr of the type we want to parse
// the associated Err type has to implement something
// possibly Clone

// Some fields of EncodeArgs don't have the macro arg, that might be a problem?
use crate::chunk_type::ChunkType; // Chunk type has FromStr defined
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Pngme")]
#[command(author = "dominik.godek@gmail.com")]
#[command(version = "1.0")]
#[command(about = "Hides messages in PNG files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a hidden message
    Encode(EncodeArgs),
    /// Decode a hidden message
    Decode(DecodeArgs),
    /// Remove a hidden message
    Remove(RemoveArgs),
    /// Print all chunks (I don't know if that is what they intend me to do)
    Print(PrintArgs),
}

#[derive(Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    #[arg(value_parser = clap::value_parser!(ChunkType))]
    pub chunk_type: ChunkType,
    pub message: String,
    #[arg(default_value = PathBuf::from("out.png").into_os_string())]
    pub output_file: PathBuf
} 

#[derive(Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    #[arg(value_parser = clap::value_parser!(ChunkType))]
    pub chunk_type: ChunkType,
}

#[derive(Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    #[arg(value_parser = clap::value_parser!(ChunkType))]
    pub chunk_type: ChunkType,
}

#[derive(Args)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
