use crate::types::WebsiteSupersetReport;
use crate::types::WebsiteCommentTypeReport;
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

fn clean_website_id (id: String) -> String {
    if id.ends_with(".0") {
        let trimmed = &id[..id.len() -2];
        return trimmed.to_string();
    }
    id
}

pub fn having_type_or_comment(_data: &[WebsiteCommentTypeReport]) -> Vec<WebsiteCommentTypeReport> {
    return _data.iter().filter(|&x| has_at_least_one_property_not_null(x)).cloned().collect();
}

fn has_at_least_one_property_not_null(x: &WebsiteCommentTypeReport) -> bool {
    !x.comment.is_empty() || !x.publication_type.is_empty()
}
