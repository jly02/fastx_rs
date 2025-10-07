use clap::Parser;
use std::fs::File;
use std::io;
use fastaq_parser::parse_fasta;

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
    for (i, c) in v.iter().enumerate() {
        println!("Sequence {}: {}", i + 1, c);
    }
}
