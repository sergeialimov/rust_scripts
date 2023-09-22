use crate::types::WebsiteSupersetReport;
use std::collections::HashMap;

pub fn website_id_superset_report_map(input_data:&[WebsiteSupersetReport]) -> HashMap<String, WebsiteSupersetReport> {
    let mut map = HashMap::new();

    for item in input_data {
        map.insert(item.website_id.clone(), item.clone());
    }

    map
}

pub fn clean_website_ids (input_data: &[WebsiteSupersetReport]) -> Vec<WebsiteSupersetReport> {
    let mut res = Vec::new();

    for item in input_data {
        let mut new_item = item.clone();
        new_item.website_id = clean_website_id(item.website_id.clone());


        
        res.push(new_item);
    }

    res
}

pub fn clean_website_id (id: String) -> String {
    if id.ends_with(".0") {
        let trimmed = &id[..id.len() -2];
        return trimmed.to_string();
    }
    id
}