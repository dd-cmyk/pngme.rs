// TODO: rewrite the 'decode' function
#![allow(dead_code)] // Get rid of 'is never used' warnings in the entire crate
mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use args::Commands;
use clap::Parser;
use commands::{decode, encode, print, remove};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();
    if let Commands::Encode(args) = &cli.command {
        println!("{:?}", args.output_file);
    }

    match &cli.command {
        Commands::Encode(args) => encode(&args.file_path, &args.chunk_type, &args.message, &args.output_file), 
        Commands::Decode(args) => decode(&args.file_path, &args.chunk_type),
        Commands::Remove(args) => remove(&args.file_path, &args.chunk_type),
        Commands::Print(args) => print(&args.file_path),
    }
}
