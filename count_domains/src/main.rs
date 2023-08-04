use serde::{Deserialize, Serialize};
// use serde_json::to_string;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use url::Url;
use csv::Writer;

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
    isMirror: Option<u32>,
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
struct WebsiteIdsCounter {
    website_ids: Vec<u32>,
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
    let path = "data/input/publications-tad.json";

    let data = read_json_file(path)?;

    let mut domains_counts: HashMap<String, WebsiteIdsCounter> = HashMap::new();

    for item in &data {
        let url_string = &item.publicationUrl.url;
        if !url_string.is_empty() {
            let url = Url::parse(url_string).unwrap();
            let website_id = item.websiteId;

            let domain = url.domain().unwrap_or("").to_string();

            let website_ids_counter = domains_counts
                .entry(domain.clone())
                .or_insert(WebsiteIdsCounter {
                    website_ids: Vec::new(),
                    counter: 0,
                });
            website_ids_counter.counter += 1;
            if !website_ids_counter.website_ids.contains(&website_id) {
                website_ids_counter.website_ids.push(website_id);
            }
        }
    }

    let mut domains_vec: Vec<(&String, &WebsiteIdsCounter)> = domains_counts.iter().collect();

    // Sort the Vec based on the counter field of the WebsiteIdCount struct
    domains_vec.sort_by(|a, b| b.1.counter.cmp(&a.1.counter));

    let mut wtr = Writer::from_path("data/output/domains.csv")?;

    for item in &domains_vec {
        let domain = item.0;
        let counter = item.1.counter.to_string();
        let website_ids = item.1.website_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
        wtr.write_record(&[domain, &counter, &website_ids])?;
    }

    wtr.flush()?;

    Ok(())
}
