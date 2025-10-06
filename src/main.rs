use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
struct Args {
    /// Path to the file to read
    filename: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let reader = BufReader::new(file);

    let mut seqs: Vec<String> = Vec::new();
    let mut seq = String::new();
    for line_result in reader.lines() {
        let line = line_result?;
    
        if line.starts_with(">") {
            seqs.push(seq);
            seq = String::new();
        } else {
            seq.push_str(&line);
        }
    }

    // need to add last sequence after completion
    seqs.push(seq);

    print_vec(seqs);

    Ok(())
}

fn print_vec(v: Vec<String>) {
    for comp in v {
        println!("{}", comp);
    }
}
