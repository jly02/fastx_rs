use std::fs::File;
use std::io::{BufRead, BufReader, Error};

pub(crate) fn parse_fasta(file: File) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(file);

    let mut seqs: Vec<String> = Vec::new();
    let mut seq = String::new();

    for line_result in reader.lines() {
        let line = line_result?;

        if line.starts_with('>') {
            if !seq.is_empty() {
                seqs.push(seq);
                seq = String::new();
            }
        } else {
            seq.push_str(&line);
        }
    }

    // need to add last sequence if there is one
    if !seq.is_empty() {
        seqs.push(seq);
    }

    Ok(seqs)
}