use crate::models::cosmetics::{
    Brand, BrandItem, BrandSequence, HotProduct, NewBrand, NewProduct, ProductItem,
};
use crate::models::{CommonStatus, Paging, MAX_ROWS, MIN_ROWS};
use anyhow::{anyhow, Result};
use sqlx::mysql::MySqlPool;
use sqlx::{query, query_as, query_as_unchecked, query_unchecked, Done, Row};

// brands

pub async fn create_brand(db: &MySqlPool, brand: Brand, operator: &str) -> Result<u64> {
    let id = query_unchecked!(
        r#"
INSERT INTO brand (`name`, `sequence`, `creator`)
VALUES ( ?, ?, ?)
"#,
        brand.name,
        brand.sequence,
        operator,
    )
    .execute(db)
    .await?
    .last_insert_id();

    Ok(id)
}

pub async fn is_brand_names_valid(db: &MySqlPool, brands: &Vec<NewBrand>) -> Result<bool> {
    let names: Vec<String> = brands
        .iter()
        .map(|b| r#"""#.to_string() + &b.name.clone() + r#"""#)
        .collect();
    let s = format!(
        "SELECT DISTINCT name FROM brand WHERE name IN ({}) AND status = {}",
        names.join(", "),
        CommonStatus::Valid
    );
    let rows = query(s.as_str()).fetch_all(db).await?;
    if rows.len() > 0 {
        let get_names: Vec<String> = rows.iter().map(|row| row.get(0)).collect();
        return Err(anyhow!(
            "Brand names already exist: {:?}",
            get_names.join(", ")
        ));
    }
    Ok(true)
}

pub async fn get_max_brand_sequence(db: &MySqlPool) -> Result<i32> {
    let record = query_unchecked!(
        r#"SELECT COALESCE(MAX(`sequence`), 0) AS `max_id` FROM brand WHERE status = ?"#,
        CommonStatus::Valid as i8
    )
    .fetch_one(db)
    .await?;

    Ok(record.max_id.unwrap_or(0_i64) as i32)
}

pub async fn create_brands(db: &MySqlPool, brands: Vec<Brand>, operator: &str) -> Result<bool> {
    let mut brands = brands.clone();
    brands.sort_by(|a, b| a.sequence.cmp(&b.sequence));
    let mut s = brands.iter().fold(
        "INSERT INTO brand (`name`, `sequence`, `creator`) VALUES ".to_string(),
        |s, brand| {
            format!(
                "{} ('{}', {}, '{}'),",
                s, brand.name, brand.sequence, operator
            )
        },
    );
    s.pop();
    let id = query(s.as_str()).execute(db).await?.last_insert_id();

    Ok(id > 0)
}

pub async fn get_brands(db: &MySqlPool, paging: Paging) -> Result<Vec<Brand>> {
    query_as_unchecked!(
        Brand,
        r#"
SELECT id, `name`, `sequence`, false AS is_hot
FROM brand
WHERE status = ?
ORDER BY `sequence`, id
LIMIT ?, ?"#,
        CommonStatus::Valid as i8,
        paging.offset.unwrap_or(0),
        paging.limit.unwrap_or(MAX_ROWS).min(MAX_ROWS),
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into())
}

pub async fn get_brand_id(db: &MySqlPool, brand_name: &str) -> Result<u64> {
    let record = query_unchecked!(
        r#"SELECT id FROM brand WHERE name = ? AND status = ?"#,
        brand_name,
        CommonStatus::Valid as i8
    )
    .fetch_optional(db)
    .await?;

    match record {
        Some(r) => Ok(r.id),
        None => Err(anyhow!("品牌名'{}'不存在", brand_name)),
    }
}

pub async fn get_all_brands(db: &MySqlPool) -> Result<Vec<Brand>> {
    query_as_unchecked!(
        Brand,
        r#"
SELECT id, `name`, `sequence`, false AS is_hot
FROM brand
WHERE status = ?
ORDER BY `sequence`, id"#,
        CommonStatus::Valid as i8,
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into())
}

pub async fn update_brand(db: &MySqlPool, brand: Brand, operator: &str) -> Result<bool> {
    let row = query_unchecked!(
        r#"UPDATE brand SET `name`= ?, `sequence` = ?, modifier = ? WHERE id = ?"#,
        brand.name,
        brand.sequence,
        operator,
        brand.id,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(row > 0)
}

pub async fn delete_brand(db: &MySqlPool, id: u32, operator: &str) -> Result<bool> {
    let row = query_unchecked!(
        r#"UPDATE brand SET status = ?, modifier = ? WHERE id = ?"#,
        CommonStatus::Invalid as i8,
        operator,
        id,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(row > 0)
}

pub async fn is_brand_ids_valid(db: &MySqlPool, ids: Vec<String>) -> Result<bool> {
    let s = format!(
        "SELECT id FROM brand WHERE id IN ({}) AND status = {}",
        ids.join(", "),
        CommonStatus::Valid,
    );
    let get_ids = query(s.as_str()).fetch_all(db).await?;
    if get_ids.len() != ids.len() {
        return Err(anyhow!("Brand ids not match: {:?}", ids));
    }
    Ok(true)
}

pub async fn update_brand_sequence(
    db: &MySqlPool,
    brand_sequence: &BrandSequence,
    operator: &str,
) -> Result<bool> {
    let row = query_unchecked!(
        r#"UPDATE brand SET `sequence` = ?, modifier = ? WHERE id = ?"#,
        brand_sequence.sequence,
        operator,
        brand_sequence.id,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(row > 0)
}

pub async fn get_brand_detail(db: &MySqlPool, id: u32, paging: Paging) -> Result<Vec<BrandItem>> {
    query_as!(
        BrandItem,
        r#"
SELECT `id`, `name`, `title`, `subtitle`, `img_url`
FROM product
WHERE brand_id = ? AND status = ?
ORDER BY id
LIMIT ?, ?
"#,
        id,
        CommonStatus::Valid as i8,
        paging.offset.unwrap_or(0),
        paging.limit.unwrap_or(MIN_ROWS).min(MAX_ROWS),
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into())
}

// product

pub async fn create_product(
    db: &MySqlPool,
    product: &NewProduct,
    brand_id: u64,
    operator: &str,
) -> Result<u64> {
    let id = query_unchecked!(
        r#"
INSERT INTO product (`name`, `alias`, `title`, `subtitle`, `brand_id`, `spec`,
`kind`, `sell_price`, `import_price`, `sequence`, `jd_id`, `jd_url`, `img_url`,
`status`, `comment`, `creator`)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
"#,
        product.name,
        product.alias,
        product.title,
        product.subtitle,
        brand_id,
        product.spec,
        product.kind,
        product.sell_price,
        product.import_price,
        product.sequence,
        product.jd_id,
        product.jd_url,
        product.img_url,
        product.status,
        product.comment,
        operator,
    )
    .execute(db)
    .await?
    .last_insert_id();

    Ok(id)
}

pub async fn get_valid_product(db: &MySqlPool, id: u64) -> Result<Option<ProductItem>> {
    query_as_unchecked!(
        ProductItem,
        r#"
SELECT p.id, p.name, p.alias, p.title, p.subtitle, p.brand_id, b.name as brand_name,
p.spec, p.kind, p.sell_price, p.import_price, p.sequence, p.jd_id, p.jd_url,
p.img_url, p.status, p.comment
FROM product p
JOIN brand b
ON p.brand_id = b.id
WHERE p.id = ? AND p.status = ?
"#,
        id,
        CommonStatus::Valid as i8,
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into())
}

pub async fn get_product(db: &MySqlPool, id: u64) -> Result<Option<ProductItem>> {
    query_as_unchecked!(
        ProductItem,
        r#"
SELECT p.id, p.name, p.alias, p.title, p.subtitle, p.brand_id, b.name as brand_name,
p.spec, p.kind, p.sell_price, p.import_price, p.sequence, p.jd_id, p.jd_url,
p.img_url, p.status, p.comment
FROM product p
JOIN brand b
ON p.brand_id = b.id
WHERE p.id = ?
"#,
        id,
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into())
}

pub async fn get_all_products(db: &MySqlPool, paging: Paging) -> Result<Vec<ProductItem>> {
    query_as_unchecked!(
        ProductItem,
        r#"
SELECT p.id, p.name, p.alias, p.title, p.subtitle, p.brand_id, b.name as brand_name,
p.spec, p.kind, p.sell_price, p.import_price, p.sequence, p.jd_id, p.jd_url,
p.img_url, p.status, p.comment
FROM product p
JOIN brand b
ON p.brand_id = b.id
ORDER BY p.id
LIMIT ?, ?
"#,
        paging.offset.unwrap_or(0),
        paging.limit.unwrap_or(MAX_ROWS).min(MAX_ROWS),
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into())
}

pub async fn update_product(
    db: &MySqlPool,
    id: u64,
    product: NewProduct,
    operator: &str,
) -> Result<bool> {
    let row = query_unchecked!(
        r#"
UPDATE product SET `name` = ?, `alias` = ?, `title` = ?, `subtitle` = ?,
`brand_id` = ?, `spec` = ?, `kind` = ?, `sell_price` = ?, `import_price` = ?,
`sequence` = ?, `jd_id` = ?, `jd_url` = ?, `img_url` = ?, `status` = ?,
`comment` = ?, modifier = ?
WHERE id = ?
"#,
        product.name,
        product.alias,
        product.title,
        product.subtitle,
        product.brand_id,
        product.spec,
        product.kind,
        product.sell_price,
        product.import_price,
        product.sequence,
        product.jd_id,
        product.jd_url,
        product.img_url,
        product.status,
        product.comment,
        operator,
        id,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(row > 0)
}

pub async fn delete_product(db: &MySqlPool, id: u64, operator: &str) -> Result<bool> {
    let row = query_unchecked!(
        r#"UPDATE product SET status = ?, modifier = ? WHERE id = ?"#,
        CommonStatus::Invalid as i8,
        operator,
        id,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(row > 0)
}

pub async fn is_product_valid(db: &MySqlPool, id: u64) -> Result<bool> {
    let id = query_unchecked!(
        r#"SELECT id FROM product WHERE id = ? AND status = ? LIMIT 1"#,
        id,
        CommonStatus::Valid as i8,
    )
    .fetch_optional(db)
    .await?;

    match id {
        Some(_) => return Ok(true),
        None => Ok(false),
    }
}

// hot product

pub async fn get_hot_products(db: &MySqlPool) -> Result<Vec<HotProduct>> {
    query_as_unchecked!(
        HotProduct,
        r#"
SELECT `product_id`
FROM hot_product
WHERE status = ?
ORDER BY id"#,
        CommonStatus::Valid as i8,
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into())
}

pub async fn delete_hot_products(db: &MySqlPool, operator: &str) -> Result<bool> {
    let _ = query_unchecked!(
        r#"UPDATE hot_product SET status = ?, modifier = ? WHERE status != ?"#,
        CommonStatus::Invalid as i8,
        operator,
        CommonStatus::Invalid as i8,
    )
    .execute(db)
    .await?
    .rows_affected();

    Ok(true)
}

pub async fn create_hot_products(
    db: &MySqlPool,
    hot_products: Vec<u64>,
    operator: &str,
) -> Result<bool> {
    let mut s = hot_products.iter().fold(
        "INSERT INTO hot_product (`product_id`, `creator`) VALUES ".to_string(),
        |s, hp| format!("{} ({}, '{}'),", s, hp, operator),
    );
    s.pop();
    let id = query(s.as_str()).execute(db).await?.last_insert_id();

    Ok(id > 0)
}
