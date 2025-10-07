//! This crate provides utilities for parsing FASTA and FASTQ files.
//! 
//! It includes functions for reading sequences, filtering by quality, and storing results.

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

/// Parses a FASTA file into a vector of sequences.
///
/// ## Arguments
///
/// * `file` - A `File` object pointing to a FASTA file.
///
/// ## Returns
///
/// A `Result` containing a `Vec<String>` of sequences on success,
/// or an `io::Error` on failure.
///
/// ## Example
///
/// ```rust
/// use std::fs::File;
/// let file = File::open("example.fasta").unwrap();
/// let sequences = parse_fasta(file).unwrap();
/// ```
pub fn parse_fasta(file: File) -> Result<Vec<String>, Error> {
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

pub fn parse_fastq(file: File) -> Result<Vec<String>, Error> {
    Ok(Vec::new())
}