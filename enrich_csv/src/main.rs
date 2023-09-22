mod types;
mod csv_helper;
mod service;

fn main() {
    let file_path1 = "./data/input/missing_issn.csv";
    match csv_helper::read_csv_file::<types::WebsiteCommentsInput>(file_path1) {
        Ok(_data) => {
            
            
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }

    let file_path2 = "./data/input/superset_sep_17.csv";
    match csv_helper::read_csv_file::<types::WebsiteSupersetReport>(file_path2) {
        Ok(_data) => {
            let map = service::website_id_superset_report_map(&_data);

            println!("{:?}", map.get("763"));
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }
}