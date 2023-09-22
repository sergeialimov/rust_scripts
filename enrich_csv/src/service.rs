use crate::types::WebsiteSupersetReport;
use std::collections::HashMap;

pub fn website_id_superset_report_map(input_data:&[WebsiteSupersetReport]) -> HashMap<String, WebsiteSupersetReport> {
    let mut map = HashMap::new();

    for item in input_data {
        map.insert(item.website_id.clone(), item.clone());
    }

    map
}