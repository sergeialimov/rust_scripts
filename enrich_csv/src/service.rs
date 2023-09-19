use crate::types::WebsiteCommentsInput;

use std::collections::HashMap;

pub fn create_website_id_comment_map(input_data: &[WebsiteCommentsInput]) -> HashMap<String, String> {
  let mut map = HashMap::new();

  for item in input_data {
    map.insert(item.website_id.clone(), item.comment.clone());
  }

  map
}