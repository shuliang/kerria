use warp::Filter;

use crate::environment::Environment;
use crate::handlers;
use crate::helpers::problem;
use crate::models::cosmetics::{BrandSequence, NewBrand, NewProduct};
use crate::models::Paging;

pub fn cosmetics(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let env = warp::any().map(move || env.clone());

    // cosmetics api
    let prefix = warp::path!("api" / "v1" / "cosmetics" / ..);

    // GET /api/v1/cosmetics/brands
    let get_brands = warp::path("brands")
        .and(warp::path::end())
        .and(warp::get())
        .and(env.clone())
        .and(warp::query::<Paging>())
        .and_then(|env: Environment, paging: Paging| async move {
            handlers::cosmetics::get_brands(env, paging)
                .await
                .map_err(problem::build)
        });

    // GET /api/v1/cosmetics/brand/{id}
    let get_brand_detail = warp::path!("brand" / u32)
        .and(warp::get())
        .and(env.clone())
        .and(warp::query::<Paging>())
        .and_then(|id: u32, env: Environment, paging: Paging| async move {
            handlers::cosmetics::get_brand_detail(env, id, paging)
                .await
                .map_err(problem::build)
        });

    // GET /api/v1/cosmetics/product/{id}
    let get_product_detail = warp::path!("product" / u32)
        .and(warp::get())
        .and(env.clone())
        .and_then(|id: u32, env: Environment| async move {
            handlers::cosmetics::get_product(env, id)
                .await
                .map_err(problem::build)
        });

    prefix.and(get_brands.or(get_brand_detail).or(get_product_detail))
}

pub fn admin_cosmetics(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let env = warp::any().map(move || env.clone());

    // cosmetics api
    let prefix = warp::path!("api" / "v1" / "cosmetics" / ..);

    // TODO: add operator
    // let operator = "admin";

    // brand

    // GET /../brands
    let create_brands = warp::path!("brands")
        .and(warp::path::end())
        .and(warp::post())
        .and(env.clone())
        .and(warp::body::content_length_limit(10240))
        .and(warp::body::json())
        .and_then(|env: Environment, brands: Vec<NewBrand>| async move {
            handlers::cosmetics::create_brands(env, brands, "admin")
                .await
                .map_err(problem::build)
        });

    // GET /../brands
    let get_brands = warp::path!("brands")
        .and(warp::path::end())
        .and(warp::get())
        .and(env.clone())
        .and_then(|env: Environment| async move {
            handlers::cosmetics::get_all_brands(env)
                .await
                .map_err(problem::build)
        });

    // PUT /../brands/sequence
    let update_brands_sequence = warp::path!("brands" / "sequence")
        .and(warp::path::end())
        .and(warp::put())
        .and(env.clone())
        .and(warp::body::content_length_limit(10240))
        .and(warp::body::json())
        .and_then(|env: Environment, bss: Vec<BrandSequence>| async move {
            handlers::cosmetics::update_brands_sequence(env, bss, "admin")
                .await
                .map_err(problem::build)
        });

    // DELETE /../brand/{id}
    let delete_brand = warp::path!("brand" / u32)
        .and(warp::delete())
        .and(env.clone())
        .and_then(|id: u32, env: Environment| async move {
            handlers::cosmetics::delete_brand(env, id, "admin")
                .await
                .map_err(problem::build)
        });

    let api_brands = create_brands
        .or(get_brands)
        .or(update_brands_sequence)
        .or(delete_brand);

    // product

    // POST /../product
    let create_product = warp::path!("product")
        .and(warp::path::end())
        .and(env.clone())
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and_then(|env: Environment, product: NewProduct| async move {
            handlers::cosmetics::create_product(env, product, "admin")
                .await
                .map_err(problem::build)
        });

    // GET /../products
    let get_product_list = warp::path!("products")
        .and(warp::path::end())
        .and(warp::get())
        .and(env.clone())
        .and(warp::query::<Paging>())
        .and_then(|env: Environment, paging: Paging| async move {
            handlers::cosmetics::get_products(env, paging)
                .await
                .map_err(problem::build)
        });

    // GET /../product/{id}
    let get_product = warp::path!("product" / u32)
        .and(warp::get())
        .and(env.clone())
        .and_then(|id: u32, env: Environment| async move {
            handlers::cosmetics::get_product(env, id)
                .await
                .map_err(problem::build)
        });

    // PUT /../product/{id}
    let update_product = warp::path!("product" / u32)
        .and(warp::put())
        .and(env.clone())
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and_then(
            |id: u32, env: Environment, product: NewProduct| async move {
                handlers::cosmetics::update_product(env, id, product, "admin")
                    .await
                    .map_err(problem::build)
            },
        );

    // DELETE /../product/{id}
    let delete_product = warp::path!("product" / u32)
        .and(warp::delete())
        .and(env.clone())
        .and_then(|id: u32, env: Environment| async move {
            handlers::cosmetics::delete_product(env, id, "admin")
                .await
                .map_err(problem::build)
        });

    let api_products = create_product
        .or(get_product_list)
        .or(get_product)
        .or(update_product)
        .or(delete_product);

    // hot product

    // POST /../product/hot
    let add_hot_product = warp::path!("product" / "hot")
        .and(warp::path::end())
        .and(warp::post())
        .and(env.clone())
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and_then(|env: Environment, hps: Vec<u32>| async move {
            handlers::cosmetics::add_hot_product(env, hps, "admin")
                .await
                .map_err(problem::build)
        });

    // GET /../product/hot
    let get_hot_products = warp::path!("product" / "hot")
        .and(warp::path::end())
        .and(warp::get())
        .and(env.clone())
        .and_then(|env: Environment| async move {
            handlers::cosmetics::get_hot_products(env)
                .await
                .map_err(problem::build)
        });

    let api_hot_products = add_hot_product.or(get_hot_products);

    prefix.and(api_brands.or(api_products).or(api_hot_products))
}
