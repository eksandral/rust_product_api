use std::fs;

use crate::model::{Discount, Product};

#[derive(Debug)]
pub struct DB {
    products: Vec<Product>,
    discounts: Vec<Discount>,
}

impl DB {
    pub fn new() -> Self {
        let path_to_products = std::env::var("DB_PRODUCTS").unwrap_or("data/products.json".to_string());
        let data = fs::read(path_to_products).expect("Cannot read product JSON file");
        let products = serde_json::from_slice(&data).expect("Cannot load product data");

        let path_to_discounts =
            std::env::var("DB_DISCOUNTS").unwrap_or("data/discounts.json".to_string());
        let data = fs::read(path_to_discounts).expect("Cannot read product JSON file");
        let discounts = serde_json::from_slice(&data).expect("Cannot load product data");
        Self {
            products,
            discounts,
        }
    }
    pub fn get_products(&self) -> &Vec<Product> {
        &self.products
    }
    pub fn get_discounts(&self) -> &Vec<Discount> {
        &self.discounts
    }
}
