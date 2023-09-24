mod types;
mod csv_helper;
mod service;

use std::collections::HashMap;

fn main() {
    use crate::types::{ WebsiteCommentTypeReport, WebsiteSupersetReport };
    let file_path1 = "./data/input/comment_type_report.csv";
    let mut comment_type_report: Vec<WebsiteCommentTypeReport> = Vec::new();
    match csv_helper::read_csv_file::<WebsiteCommentTypeReport>(file_path1) {
        Ok(_data) => {
            comment_type_report = service::having_type_or_comment(&_data);
            // println!("{:?}", comment_type_report[0]);
            println!("{:?}", comment_type_report.len());
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }

    let file_path2 = "./data/input/superset_sep_19_2.csv";
    let mut map = HashMap::new();
    match csv_helper::read_csv_file::<WebsiteSupersetReport>(file_path2) {
        Ok(_data) => {
            let cleaned_data = service::clean_website_ids(&_data);
            map = service::website_id_superset_report_map(&cleaned_data);
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }

    let iter = comment_type_report.iter();

    for val in iter {
        if let Some(mut website_data) = map.get_mut(&val.website_id).cloned() {
            if !val.publication_type.is_empty() {
                website_data.publication_type = val.publication_type.clone() ;
            }
            if !val.comment.is_empty() {
                website_data.comment = val.comment.clone() ;
            }
            map.insert(val.website_id.clone(), website_data);
        }
    }

    // let map_values = map.values().collect();
    let map_values: Vec<WebsiteSupersetReport> = map.values().cloned().collect();

    let _res = csv_helper::write_csv_path(&map_values, "./data/output/enriched.csv");
}
