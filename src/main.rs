use std::{fmt::Error, fs::File, io::{Read, Write}, path::PathBuf};

use clap::Parser;

/// Read a file path
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of file
    #[clap(short, long)]
    input_file : PathBuf
}

fn main() {
    let args = Args::parse();
    let mut buf : Vec<u8> = Vec::new();
    let f = File::open(args.input_file);
    match f {
        Ok(mut f) =>  {
            f.read_to_end(&mut buf);
        },
        Err(e) => panic!("{:?}", e),
    }
    std::io::stdout().write(&buf);
}
