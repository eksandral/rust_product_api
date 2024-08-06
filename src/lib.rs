use serde::Deserialize;

pub mod db;
pub mod error;
pub mod model;
pub mod response;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProductFilterParams {
    pub category: String,
    pub price_less_then: Option<u32>,
    pub page_size: Option<usize>,
    pub page_num: Option<usize>,
}
