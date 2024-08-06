use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub category: String,
    pub price: u32,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Price {
    pub original: u32,
    pub final_price: u32,
    pub discount_percentage: Option<u32>,
    pub currency: Currency,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Currency {
    EUR,
}
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Discount {
    #[serde(rename(deserialize = "category"))]
    Category { predicate: String, value: u32 },
    #[serde(rename(deserialize = "sku"))]
    Sku { predicate: String, value: u32 },
}
impl From<(&Product, &[Discount])> for Price {
    fn from((product, discounts): (&Product, &[Discount])) -> Self {
        let mut product_discounts = vec![];
        for discount in discounts {
            match discount {
                Discount::Sku { predicate, value } if predicate.as_str() == product.sku => {
                    product_discounts.push(value)
                }
                Discount::Category { predicate, value }
                    if predicate.as_str() == product.category =>
                {
                    product_discounts.push(value)
                }
                _ => {}
            }
        }
        let (discount_price, discount) = product_discounts
            .iter()
            .max()
            .map(|x| (product.price * *x / 100, Some(**x)))
            .unwrap_or((0, None));
        Price {
            original: product.price,
            final_price: product.price - discount_price,
            discount_percentage: discount,
            currency: Currency::EUR,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discount_sku_applied() {
        const DISCOUNT: u32 = 15;
        let product = Product {
            sku: "1".to_string(),
            category: "category 1".to_string(),
            name: "Product 1".to_string(),
            price: 30000,
        };
        let discounts = vec![
            Discount::Category {
                predicate: "1".to_string(),
                value: 35,
            },
            Discount::Sku {
                predicate: "1".to_string(),
                value: DISCOUNT,
            },
        ];
        let price: Price = (&product, &discounts[..]).into();
        assert_eq!(
            price,
            Price {
                original: product.price,
                final_price: product.price - product.price * DISCOUNT / 100,
                discount_percentage: Some(DISCOUNT),
                currency: Currency::EUR
            }
        );
    }
    #[test]
    fn discount_category_applied() {
        const DISCOUNT: u32 = 35;
        let product = Product {
            sku: "1".to_string(),
            category: "category 1".to_string(),
            name: "Product 1".to_string(),
            price: 30000,
        };
        let discounts = vec![
            Discount::Category {
                predicate: "category 1".to_string(),
                value: DISCOUNT,
            },
            Discount::Sku {
                predicate: "2".to_string(),
                value: 15,
            },
        ];
        let price: Price = (&product, &discounts[..]).into();
        assert_eq!(
            price,
            Price {
                original: product.price,
                final_price: product.price - product.price * DISCOUNT / 100,
                discount_percentage: Some(DISCOUNT),
                currency: Currency::EUR
            }
        );
    }
    #[test]
    fn discount_max_applied() {
        const DISCOUNT: u32 = 35;
        let product = Product {
            sku: "1".to_string(),
            category: "category 1".to_string(),
            name: "Product 1".to_string(),
            price: 30000,
        };
        let discounts = vec![
            Discount::Category {
                predicate: "category 1".to_string(),
                value: DISCOUNT,
            },
            Discount::Sku {
                predicate: "1".to_string(),
                value: DISCOUNT - 10,
            },
        ];
        let price: Price = (&product, &discounts[..]).into();
        assert_eq!(
            price,
            Price {
                original: product.price,
                final_price: product.price - product.price * DISCOUNT / 100,
                discount_percentage: Some(DISCOUNT),
                currency: Currency::EUR
            }
        );
    }
}
