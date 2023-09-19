mod types;
mod csv_helper;
mod service;

fn main() {
    let file_path1 = "./data/input/missing_issn.csv";
    match csv_helper::read_csv_file::<types::WebsiteCommentsInput>(file_path1) {
        Ok(data) => {
            let map = service::create_website_id_comment_map(&data);
            print!("{:?}", map.get("79657"));
        }
        Err(err) => {
            eprintln!("Error reading CSV: {}", err);
        }
    }

    // let file_path2 = "./data/input/superset_sep_17.csv";
    // match csv_helper::read_csv_file::<types::WebsiteSupersetReport>(file_path2) {
    //     Ok(data) => {
    //         print!("{:?}", data[0]);
    //     }
    //     Err(err) => {
    //         eprintln!("Error reading CSV: {}", err);
    //     }
    // }
}