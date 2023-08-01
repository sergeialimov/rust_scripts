use serde::Deserialize;
// use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader};
use std::path::Path;
use std::collections::HashMap;
use url::Url;


#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
////test
struct Publication {
    // issnL: Option<String>,
    // publicationName: String,
    // advertiserTag: u32,
    // publicationType: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct PublicationUrl {
    url: String,
    // isMirror: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct Data {
    // publication: Publication,
    publicationUrl: PublicationUrl,
    // issns: Vec<String>,
    // websiteId: u32,
}

fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Vec<Data>, Box<dyn std::error::Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // Read the JSON contents of the file as an instance of `Data`.
    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data/input/publications_to_review.json";

    let data = read_json_file(path)?;

    let mut domain_counts: HashMap<String, i32> = HashMap::new();

    for item in &data {
        let url_string = &item.publicationUrl.url;
        if !url_string.is_empty() {
            let url = Url::parse(url_string).unwrap();
            let domain = url.domain().unwrap_or("");
            let counter = domain_counts.entry(domain.to_string()).or_insert(0);
            *counter += 1;
        }
    }

    // Convert the HasMap into a Vec of (domain, count) tuples
    let mut domain_counts_vec: Vec<(_, _)> = domain_counts.into_iter().collect();

    // Sort the Vec in descending order by count
    domain_counts_vec.sort_by(|a, b|b.1.cmp(&a.1));

    // Open the output file in write mode, create it if if does not exist
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("data/output/domains.txt")?;

    for (domain, count) in domain_counts_vec {
        writeln!(file, "\"{}\": {}", domain, count)?;
    }
    
    Ok(())
}
