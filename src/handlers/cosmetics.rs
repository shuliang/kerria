use crate::environment::Environment;
use crate::models::cosmetics::{Brand, BrandSequence, NewBrand, NewProduct};
use crate::models::{Paging, RespData, Validate};
use crate::sql;
use anyhow::{anyhow, Result};
use serde_json::json;
use warp::http::StatusCode;

// brand

pub async fn create_brand(
    env: Environment,
    brand: Brand,
    operator: &str,
) -> Result<impl warp::Reply> {
    let res = sql::cosmetics::create_brand(env.db(), brand, operator).await?;
    Ok(warp::reply::json(&json!({ "id": res })))
}

pub async fn create_brands(
    env: Environment,
    brands: Vec<NewBrand>,
    operator: &str,
) -> Result<impl warp::Reply> {
    if brands.is_empty() {
        return Err(anyhow!("Empty brands.").into());
    }
    for b in brands.iter() {
        b.validate()?;
    }
    sql::cosmetics::is_brand_names_valid(env.db(), &brands).await?;
    let mut max_sequence = sql::cosmetics::get_max_brand_sequence(env.db()).await?;
    let new_brands: Vec<Brand> = brands
        .iter()
        .map(|b| {
            max_sequence += 1;
            Brand {
                id: 0,
                name: b.name.clone(),
                sequence: max_sequence,
                is_hot: false,
            }
        })
        .collect();
    let ok = sql::cosmetics::create_brands(env.db(), new_brands, operator).await?;
    if ok {
        return Ok(StatusCode::CREATED);
    }
    Err(anyhow!("Create brands failed.").into())
}

pub async fn get_brands(env: Environment, paging: Paging) -> Result<impl warp::Reply> {
    let res = sql::cosmetics::get_brands(env.db(), paging).await?;
    let reply = warp::reply::json(&RespData {
        total: res.len(),
        data: res,
    });
    Ok(reply)
}

pub async fn get_all_brands(env: Environment) -> Result<impl warp::Reply> {
    let res = sql::cosmetics::get_all_brands(env.db()).await?;
    let reply = warp::reply::json(&RespData {
        total: res.len(),
        data: res,
    });
    Ok(reply)
}

pub async fn get_brand_detail(
    env: Environment,
    id: u32,
    paging: Paging,
) -> Result<impl warp::Reply> {
    let res = sql::cosmetics::get_brand_detail(env.db(), id, paging).await?;
    let reply = warp::reply::json(&RespData {
        total: res.len(),
        data: res,
    });
    Ok(reply)
}

pub async fn update_brands_sequence(
    env: Environment,
    bss: Vec<BrandSequence>,
    operator: &str,
) -> Result<impl warp::Reply> {
    if bss.len() == 1 {
        return Ok(warp::reply::reply());
    }
    let ids: Vec<String> = bss.iter().map(|bs| bs.id.to_string()).collect();
    sql::cosmetics::is_brand_ids_valid(env.db(), ids).await?;

    // bss.iter().map(|&bs| async move {
    //     sql::cosmetics::update_brand_sequence(env.db(), bs, operator).await?
    // });

    for bs in bss.iter() {
        sql::cosmetics::update_brand_sequence(env.db(), bs, &operator).await?;
    }

    Ok(warp::reply())
}

pub async fn delete_brand(env: Environment, id: u32, operator: &str) -> Result<impl warp::Reply> {
    let ok = sql::cosmetics::delete_brand(env.db(), id, operator).await?;
    if ok {
        return Ok(StatusCode::NO_CONTENT);
    }
    Err(anyhow!("Delete brand failed, id: {}", id).into())
}

// product

pub async fn create_product(
    env: Environment,
    product: NewProduct,
    operator: &str,
) -> Result<impl warp::Reply> {
    product.validate()?;
    let id = sql::cosmetics::create_product(env.db(), product, operator).await?;
    let reply = warp::reply::json(&json!({ "id": id }));
    let reply = warp::reply::with_status(reply, StatusCode::CREATED);
    Ok(reply)
}

pub async fn get_product(env: Environment, id: u64) -> Result<Box<dyn warp::Reply>> {
    let res = sql::cosmetics::get_product(env.db(), id).await?;
    match res {
        Some(product) => return Ok(Box::new(warp::reply::json(&product))),
        None => return Ok(Box::new(StatusCode::OK)),
    }
}

pub async fn get_products(env: Environment, paging: Paging) -> Result<impl warp::Reply> {
    let res = sql::cosmetics::get_products(env.db(), paging).await?;
    let reply = warp::reply::json(&RespData {
        total: res.len(),
        data: res,
    });
    Ok(reply)
}

pub async fn update_product(
    env: Environment,
    id: u64,
    product: NewProduct,
    operator: &str,
) -> Result<impl warp::Reply> {
    let is_exist = sql::cosmetics::is_product_valid(env.db(), id).await?;
    if !is_exist {
        return Err(anyhow!("Delete Failed, not exist, id: {}", id));
    }
    let ok = sql::cosmetics::update_product(env.db(), id, product, operator).await?;
    if ok {
        return Ok(StatusCode::OK);
    }
    Err(anyhow!("Update product failed, id: {}.", id).into())
}

pub async fn delete_product(env: Environment, id: u64, operator: &str) -> Result<impl warp::Reply> {
    let ok = sql::cosmetics::delete_product(env.db(), id, operator).await?;
    if ok {
        return Ok(StatusCode::NO_CONTENT);
    }
    Err(anyhow!("Delete product failed, id: {}", id).into())
}

pub async fn add_hot_product(
    env: Environment,
    hot_products: Vec<u64>,
    operator: &str,
) -> Result<impl warp::Reply> {
    sql::cosmetics::delete_hot_products(env.db(), operator).await?;
    if !hot_products.is_empty() {
        sql::cosmetics::create_hot_products(env.db(), hot_products, operator).await?;
    }
    Ok(warp::reply())
}

pub async fn get_hot_products(env: Environment) -> Result<impl warp::Reply> {
    let res = sql::cosmetics::get_hot_products(env.db()).await?;
    let reply = warp::reply::json(&RespData {
        total: res.len(),
        data: res,
    });
    Ok(reply)
}
