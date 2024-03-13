use std::{fmt::Error, fs::File, io::{Read, Write}, path::PathBuf};

use clap::Parser;

/// Read a file path
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// Name of file
    #[clap(short, long)]
    input_file : PathBuf,
    #[clap(short, long)]
    extract_json : bool
}

enum GLTFChunkTypes {
    JSON = 0x4E4F534A,
    BIN = 0x004E4942,
}

#[derive(Debug)]
struct GLTFHeader {
    // Spec says everything is to be little endian
    magic : u32,
    version : u32,
    length : u32,
}

#[derive(Debug)]
struct GLTFChunk {
    chunk_length : u32,
    chunk_type : u32, // Don't convert to CHUNK_TYPE here, defer to view
    chunk_data : Vec<u8>
}

#[derive(Debug)]
struct GLTFFile {
    header : GLTFHeader,
    chunks : Vec<GLTFChunk>,
}

fn parse_gltf_file (bs : &Vec<u8>) -> GLTFFile {
    let mut index: usize = 0;
    let header = parse_gltf_header(bs);
    let mut chunks : Vec<GLTFChunk> = Vec::new();
    index += 12;
    while index < (header.length as usize) {
        let chunk : GLTFChunk = parse_gltf_chunk(bs[index..].into());
        index += (chunk.chunk_length as usize)+8;
        chunks.push(chunk);
    }
    GLTFFile {
        header : header,
        chunks: chunks
    }
}

fn parse_gltf_header (bs : &Vec<u8>) -> GLTFHeader {
    let magic = u32::from_le_bytes(bs[0..4].try_into().unwrap());
    let version = u32::from_le_bytes(bs[4..8].try_into().unwrap());
    let length = u32::from_le_bytes(bs[8..12].try_into().unwrap());

    GLTFHeader {
        magic : magic,
        version : version,
        length : length
    }
}

fn parse_gltf_chunk (bs : Vec<u8>) -> GLTFChunk {
    let chunk_length = u32::from_le_bytes(bs[0..4].try_into().unwrap());
    let chunk_type = u32::from_le_bytes(bs[4..8].try_into().unwrap());
    let data : Vec<u8> = bs[8..(8+chunk_length as usize)].into();

    GLTFChunk {
        chunk_length: chunk_length,
        chunk_type: chunk_type,
        chunk_data: data
    }
}

fn main() {
    let args = Args::parse();
    let mut buf : Vec<u8> = Vec::new();
    let f = File::open(args.input_file);
    match f {
        Ok(mut f) =>  {
            let _ = f.read_to_end(&mut buf);
        },
        Err(e) => panic!("{:?}", e),
    }
    let gltf = parse_gltf_file(&buf);
    if args.extract_json {
        std::io::stdout().write(&gltf.chunks[0].chunk_data);
    }
}
