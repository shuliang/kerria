use warp::Filter;

use crate::environment::Environment;
use crate::handlers;
use crate::helpers::problem;
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
    let get_product_detail = warp::path!("product" / u64)
        .and(warp::get())
        .and(env.clone())
        .and_then(|id: u64, env: Environment| async move {
            handlers::cosmetics::get_product(env, id, true)
                .await
                .map_err(problem::build)
        });

    prefix.and(get_brands.or(get_brand_detail).or(get_product_detail))
}
