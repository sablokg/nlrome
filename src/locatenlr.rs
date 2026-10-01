use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/*
Gaurav Sablok
gsablok@proton.me
 */

#[derive(Debug, Clone)]
pub struct SequenceRecord {
    pub id: String,
    pub sequence: String,
    pub repeat_locator: Vec<String>,
    pub fraction_length: Vec<(usize, usize)>,
    pub fraction_length_coverage: f64,
}

pub fn nlrlocateread<P: AsRef<Path>>(
    long_read_file: P,
    nlrstretch: Option<&str>,
) -> io::Result<Vec<SequenceRecord>> {
    let stretch_type = match nlrstretch {
        Some("stretch") => nlrstretch.unwrap(),
        _ => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid NLR"));
        }
    };

    let pattern = match stretch_type {
        "stretch" => r"[nlrstretch]",
        _ => unreachable!(),
    };
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    let file = File::open(&long_read_file)?;
    let reader = BufReader::new(file);
    let mut long_read_conversion = vec![];
    let mut current_id = None;
    let mut current_seq = String::new();

    for line in reader.lines() {
        let line = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('>') {
            if let Some(id) = current_id.take() {
                long_read_conversion.push((id, current_seq));
                current_seq = String::new();
            }
            current_id = Some(line);
        } else if let Some(_) = current_id.as_ref() {
            current_seq.push_str(&line);
        }
    }
    if let Some(id) = current_id {
        if !current_seq.is_empty() {
            long_read_conversion.push((id, current_seq));
        }
    }
    let mut records = vec![];
    for (id, sequence) in long_read_conversion {
        let repeat_locator: Vec<String> = re
            .find_iter(&sequence)
            .map(|m| m.as_str().to_string())
            .collect();
        let mut unique_repeats = repeat_locator.clone();
        unique_repeats.sort();
        unique_repeats.dedup();
        let fraction_length: Vec<(usize, usize)> = unique_repeats
            .iter()
            .map(|repeat| {
                let start = sequence.find(repeat).unwrap_or(0);
                (start, start + repeat.len())
            })
            .collect();

        let sequence_length = sequence.len() as f64;
        let fraction_length_sum: usize =
            fraction_length.iter().map(|(start, end)| end - start).sum();
        let fraction_length_coverage = if fraction_length_sum > 0 {
            sequence_length / fraction_length_sum as f64
        } else {
            0.0
        };

        records.push(SequenceRecord {
            id,
            sequence,
            repeat_locator,
            fraction_length,
            fraction_length_coverage,
        });
    }

    Ok(records)
}
