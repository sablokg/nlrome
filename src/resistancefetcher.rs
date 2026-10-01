use reqwest::blocking::get;
use scraper::{Html, Selector};

/*
Gaurav Sablok
gsablok@proton.me
 */

#[tokio::main]
pub async fn prgdb_sequence_fetcher(
    id: &str,
    arg_type: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://www.prgdb.org/prgdb/genes/type/reference/{}", id);
    let response = get(url).expect("string not found");
    let document = Html::parse_document(&response.text().expect("text not found"));
    let script_selector = Selector::parse("script[type='text/javascript']").unwrap();
    let scripts: Vec<_> = document.select(&script_selector).collect();
    let sequences: Vec<String> = scripts
        .iter()
        .filter_map(|script| {
            let text = script.inner_html();
            if text.contains(">") && text.contains(";") {
                let parts: Vec<&str> = text.split('>').collect();
                if parts.len() > 1 {
                    let inner_part = parts[1];
                    let paren_parts: Vec<&str> = inner_part.split('(').collect();
                    if paren_parts.len() > 1 {
                        let sequence = paren_parts[1]
                            .split(')')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        if !sequence.is_empty() {
                            return Some(sequence);
                        }
                    }
                }
            }
            None
        })
        .collect();

    if sequences.is_empty() {
        panic!("not found");
    }

    if arg_type == "dna_sequence" {
        let dna = sequences.get(0).cloned().expect("DNA sequence not found");
        println!("The dna sequence is :{}", dna);
    }
    if arg_type == "protein_sequence" {
        let protein = sequences
            .get(1)
            .cloned()
            .expect("Protein sequence not found");
        println!("The dna sequence is :{}", protein);
    }
    Ok("The sequences have been printed".to_string())
}

/*

using anyhow for threaded machine learning

use anyhow::{Context, Result};
use reqwest::blocking::get;
use scraper::{Html, Selector};

/*
Author Gaurav Sablok,
Email: codeprog@icloud.com
*/

pub fn prgdb_sequence_fetcher(id: &str, arg_type: Option<&str>) -> Result<String> {
    if id.is_empty() {
        return Err(anyhow::anyhow!("ID cannot be empty"));
    }

    let url = format!("http://www.prgdb.org/prgdb/genes/type/reference/{}", id);
    let response = get(&url)
        .context("Failed to fetch URL")?
        .text()
        .context("Failed to read response text")?;

    let document = Html::parse_document(&response);
    let script_selector = Selector::parse("script[type='text/javascript']").unwrap();
    let scripts: Vec<_> = document.select(&script_selector).collect();

    let sequences: Vec<String> = scripts
        .iter()
        .filter_map(|script| {
            let text = script.inner_html();
            if text.contains(">") && text.contains(";") {
                let parts: Vec<&str> = text.split('>').collect();
                if parts.len() > 1 {
                    let inner_part = parts[1];
                    let paren_parts: Vec<&str> = inner_part.split('(').collect();
                    if paren_parts.len() > 1 {
                        let sequence = paren_parts[1]
                            .split(')')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_string();
                        if !sequence.is_empty() {
                            return Some(sequence);
                        }
                    }
                }
            }
            None
        })
        .collect();

    if sequences.is_empty() {
        return Err(anyhow::anyhow!("No sequences found in the page"));
    }

    match arg_type {
        Some("dna_sequence") => sequences.get(0).cloned().context("DNA sequence not found"),
        Some("protein_sequence") => sequences
            .get(1)
            .cloned()
            .context("Protein sequence not found"),
        _ => Err(anyhow::anyhow!(
            "arg_type must be 'dna_sequence' or 'protein_sequence'"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dna_sequence() {
        if let Ok(seq) = prgdb_sequence_fetcher("RGA1", Some("dna_sequence")) {
            assert!(!seq.is_empty());
            println!("DNA Sequence: {}", seq);
        }
    }

    #[test]
    fn test_protein_sequence() {
        if let Ok(seq) = prgdb_sequence_fetcher("RGA1", Some("protein_sequence")) {
            assert!(!seq.is_empty());
            println!("Protein Sequence: {}", seq);
        }
    }
}

fn outputwrite() -> Result<()> {
    let dna_seq = prgdb_sequence_fetcher("RGA1", Some("dna_sequence"))?;
    println!("DNA Sequence: {}", dna_seq);
    let protein_seq = prgdb_sequence_fetcher("RGA1", Some("protein_sequence"))?;
    println!("Protein Sequence: {}", protein_seq);
    Ok(())
}
 */
