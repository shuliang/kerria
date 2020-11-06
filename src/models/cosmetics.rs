use super::Validate;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;

#[derive(Clone, Debug, Serialize)]
pub struct Brand {
    pub id: u64,
    pub name: String,
    pub sequence: i32,
    pub is_hot: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewBrand {
    pub name: String,
}

impl Validate for NewBrand {
    fn validate(&self) -> Result<(), anyhow::Error> {
        if self.name.len() == 0 {
            return Err(anyhow!("Brand name can't be empty."));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandSequence {
    pub id: u64,
    pub sequence: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BrandItem {
    pub id: u64,
    pub name: String,
    pub title: String,
    pub subtitle: String,
    pub img_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductItem {
    pub id: u64,
    pub name: String,
    pub alias: String,
    pub title: String,
    pub subtitle: String,
    pub brand_id: u64,
    pub brand_name: String,
    pub sell_price: String,
    pub img_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewProduct {
    pub name: String,
    pub alias: String,
    pub title: String,
    pub subtitle: String,
    pub brand_id: u64,
    pub brand_name: String,
    pub sell_price: Decimal,
    pub img_url: String,
}

impl Validate for NewProduct {
    fn validate(&self) -> Result<(), anyhow::Error> {
        if self.name.len() == 0 {
            return Err(anyhow!("Product name can't be empty."));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HotProduct {
    pub product_id: u64,
}
