use clap::Parser;
use std::fs::File;
use std::io;

use crate::fastx::parse_fasta;

mod fastx;

#[derive(Parser)]
struct Args {
    // path to file
    filename: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let seqs = parse_fasta(file)?;
    print_vec(seqs);
    Ok(())
}

fn print_vec(v: Vec<String>) {
    for comp in v {
        println!("{}", comp);
    }
}
