use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use url::Url;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, unused)]
struct Publication {
    issnL: Option<String>,
    publicationName: String,
    advertiserTag: u32,
    publicationType: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, unused)]
struct PublicationUrl {
    url: String,
    isMirror: u32,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case, unused)]
struct Data {
    publication: Publication,
    publicationUrl: PublicationUrl,
    issns: Vec<String>,
    websiteId: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct WebsiteIdCount {
    website_id: u32,
    counter: u32,
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

    let mut domains_counts: HashMap<String, WebsiteIdCount> = HashMap::new();

    for item in &data {
        let url_string = &item.publicationUrl.url;
        if !url_string.is_empty() {
            let url = Url::parse(url_string).unwrap();
            let website_id = item.websiteId;

            let domain = url.domain().unwrap_or("").to_string();

            let counter = domains_counts
                .entry(domain.clone())
                .or_insert(WebsiteIdCount {
                    website_id,
                    counter: 0,
                });
            counter.counter += 1;
        }
    }

    let mut domains_vec: Vec<(&String, &WebsiteIdCount)> = domains_counts.iter().collect();

    // Sort the Vec based on the counter field of the WebsiteIdCount struct
    domains_vec.sort_by(|a, b| b.1.counter.cmp(&a.1.counter));

    // Serialize the sorted Vec
    let serialized = to_string(&domains_vec).unwrap();


    // Write the serialized data to a file
    let mut file = File::create("data/output/domains")?;
    file.write_all(serialized.as_bytes())?;

    Ok(())
}
