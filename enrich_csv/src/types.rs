use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WebsiteCommentTypeReport {
    #[serde(rename = "websiteId")]
    pub website_id: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "printIssn")]
    pub print_issn: String,

    #[serde(rename = "onlineIssn")]
    pub online_issn: String,

    #[serde(rename = "comment")]
    pub comment: String,

    #[serde(rename = "publicationType")]
    pub publication_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebsiteSupersetReport {
    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "Owner name")]
    pub owner_name: String,

    #[serde(rename = "Owner type")]
    pub owner_type: String,

    #[serde(rename = "Owner ID")]
    pub owner_id: String,

    #[serde(rename = "Full website name")]
    pub full_website_name: String,

    #[serde(rename = "Abbreviated website name")]
    pub abbreviated_website_name: String,

    #[serde(rename = "Website URL")]
    pub website_url: String,

    #[serde(rename = "Website ID")]
    pub website_id: String,

    #[serde(rename = "ISSN")]
    pub issn: String,

    #[serde(rename = "Content category")]
    pub content_category: String,

    #[serde(rename = "TrendMD staff owner")]
    pub trendmd_staff_owner: String,

    #[serde(rename = "Total outbound impressions")]
    pub total_outbound_impressions: String,

    #[serde(rename = "Current number of recommendations")]
    pub current_number_of_recommendations: String,

    #[serde(rename = "Archived")]
    pub archived: String,
}
