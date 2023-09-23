mod types;
mod csv_helper;
mod service;

fn main() {
    use crate::types::{ WebsiteCommentTypeReport, WebsiteSupersetReport };
    let file_path1 = "./data/input/missing_issn.csv";
    let comment_type_report: Vec<WebsiteCommentTypeReport>;
    match csv_helper::read_csv_file::<WebsiteCommentTypeReport>(file_path1) {
        Ok(_data) => {
            comment_type_report = service::having_type_or_comment(_data);
            
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }

    let file_path2 = "./data/input/superset_sep_17.csv";
    match csv_helper::read_csv_file::<WebsiteSupersetReport>(file_path2) {
        Ok(_data) => {
            let cleaned_data = service::clean_website_ids(&_data);

            let map = service::website_id_superset_report_map(&cleaned_data);

            println!("{:?}", map.get("763"));
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }
}