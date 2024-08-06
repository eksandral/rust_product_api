use serde::Serialize;

use crate::model::Price;

#[derive(Debug, Serialize, Clone, Default)]
pub struct ProductList {
    pub products: Vec<ProductRespose>,
    pub pagination: Pagination,
}
#[derive(Debug, Serialize, Clone)]
pub struct ProductRespose {
    pub sku: String,
    pub name: String,
    pub category: String,
    pub price: Price,
}
#[derive(Debug, Serialize, Clone, Default)]
pub struct Pagination {
    pub page_num: usize,
    pub page_size: usize,
}

impl From<(usize, usize)> for Pagination {
    fn from((page_num, page_size): (usize, usize)) -> Self {
        Self {
            page_num,
            page_size,
        }
    }
}
