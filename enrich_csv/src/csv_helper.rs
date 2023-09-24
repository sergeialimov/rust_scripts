use std::error::Error;
use std::fs::File;
use serde::de::DeserializeOwned;
use serde::Serialize;
use csv::ReaderBuilder;
// use std::path::Path;

pub fn read_csv_file<T>(file_path: &str) -> Result<Vec<T>, Box<dyn Error>>
where
    T: DeserializeOwned, // Use DeserializeOwned here
{
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result?;
        records.push(record);
    }

    Ok(records)
}


pub fn write_csv_path<T: Serialize + DeserializeOwned>(
    data: &[T],
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut writer = csv::Writer::from_writer(file);
    
    for record in data {
        writer.serialize(record)?;
    }

    writer.flush()?;

    Ok(())
}