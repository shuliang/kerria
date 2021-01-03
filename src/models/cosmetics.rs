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

#[derive(Clone, Debug, Serialize)]
pub struct ProductItem {
    pub id: u64,
    pub name: String,
    pub alias: String,
    pub title: String,
    pub subtitle: String,
    pub brand_id: u32,
    pub brand_name: String,
    pub spec: String,
    pub kind: u8,
    pub sell_price: Decimal,
    pub import_price: Decimal,
    pub sequence: i32,
    pub jd_id: String,
    pub jd_url: String,
    pub img_url: String,
    pub status: u8,
    pub comment: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct NewProduct {
    pub id: Option<u64>,
    pub name: String,
    pub alias: String,
    pub title: String,
    pub subtitle: String,
    pub brand_name: String,
    pub spec: String,
    pub kind: u8,
    pub sell_price: Decimal,
    pub import_price: Decimal,
    pub sequence: i32,
    pub jd_id: String,
    pub jd_url: String,
    pub status: u8,
    pub comment: String,

    #[serde(skip_deserializing)]
    pub img_url: String,
    #[serde(skip_deserializing)]
    pub brand_id: u64,
}

impl Validate for NewProduct {
    fn validate(&self) -> Result<(), anyhow::Error> {
        if self.name.len() == 0 {
            return Err(anyhow!("商品名称不能为空"));
        }
        if self.title.len() == 0 {
            return Err(anyhow!("商品标题不能为空"));
        }
        if self.sell_price < Decimal::new(0, 2) {
            return Err(anyhow!("商品售价应为正数"));
        }
        if self.status != 0 && self.status != 1 {
            return Err(anyhow!("请检查商品状态"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct HotProduct {
    pub product_id: u64,
}
