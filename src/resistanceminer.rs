use regex::Regex;
use reqwest::blocking::get;
use scraper::{Html, Selector};

/*
Gaurav Sablok
gsablok@proton.me
 */

#[tokio::main]
pub async fn mine_resistance_genes(id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resistance_gene = id;
    let url = format!(
        "http://www.prgdb.org/prgdb/genes/type/reference/{}",
        resistance_gene
    );
    let response = get(&url)?;
    let body = response.text()?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("div.row.container").unwrap();
    let divs: Vec<_> = document.select(&selector).collect();
    let genbank_divs: Vec<_> = divs
        .into_iter()
        .filter(|div| div.text().collect::<Vec<_>>().join("").contains("GenBank"))
        .collect();
    let mut genbank_content = String::new();
    for div in genbank_divs {
        genbank_content.push_str(&div.text().collect::<Vec<_>>().join(""));
    }
    let words: Vec<&str> = genbank_content.split_whitespace().collect();
    let genbank_words: Vec<&str> = words
        .into_iter()
        .filter(|word| word.contains("GenBank"))
        .collect();
    let re = Regex::new(r"[0-9]")?;
    let mut all_digits = String::new();
    for word in genbank_words {
        for digit in re.find_iter(word) {
            all_digits.push_str(&digit.as_str());
        }
    }

    Ok(all_digits)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mine_resistance_genes() {
        if let Ok(result) = mine_resistance_genes("R001") {
            println!("GenBank ID: {}", result);
            assert!(!result.is_empty());
        }
    }
}
