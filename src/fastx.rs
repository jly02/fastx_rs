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
/// ```ignore
/// let file = File::open("example.fasta")?;
/// let seqs = parse_fasta(file)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_fasta_basic() {
        // create temp file for testing
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            ">seq1\nACGTACGT\n>seq2\nTTGGCCAA\n"
        )
        .unwrap();

        let file = tmp.reopen().unwrap();
        let seqs = parse_fasta(file).unwrap();

        assert_eq!(seqs.len(), 2);
        assert_eq!(seqs[0], "ACGTACGT");
        assert_eq!(seqs[1], "TTGGCCAA");
    }

    #[test]
    fn test_parse_fasta_multiline_sequence() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            ">seq1\nACGT\nACGT\n>seq2\nTTGG\nCCAA\n"
        )
        .unwrap();

        let file = tmp.reopen().unwrap();
        let seqs = parse_fasta(file).unwrap();

        // multi-line sequences should concatenate
        assert_eq!(seqs[0], "ACGTACGT");
        assert_eq!(seqs[1], "TTGGCCAA");
    }
}
