use warp::Filter;

use crate::environment::Environment;
use crate::handlers;
use crate::helpers::problem;
use crate::models::admin::UpdatePassword;
use crate::models::admin::{AdminLoginRequest, AdminUser};
use crate::models::cosmetics::*;
use crate::models::Paging;

pub fn admin_filters(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    admin_login(env.clone())
        .or(admin_cosmetics(env.clone()))
        .or(admin_create_user(env.clone()))
        .or(admin_current_user(env.clone()))
        .or(admin_update_password(env.clone()))
}

fn admin_login(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let env = warp::any().map(move || env.clone());
    warp::path!("admin" / "api" / "v1" / "login")
        .and(warp::post())
        .and(env.clone())
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(|env: Environment, req: AdminLoginRequest| async move {
            handlers::admin::login_handler(env, req)
                .await
                .map_err(problem::build)
        })
}

fn with_auth(
    env: Environment,
) -> impl Filter<Extract = (Environment, AdminUser), Error = warp::Rejection> + Clone {
    let env = warp::any().map(move || env.clone());
    let auth = warp::header::optional::<String>("authorization")
        .and(env.clone())
        .and_then(|jwt_raw: Option<String>, env: Environment| async move {
            env.jwt()
                .decode_to_admin_user(jwt_raw)
                .map_err(problem::build)
        });

    warp::any().and(env.clone()).and(auth)
}

fn admin_create_user(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin" / "gen")
        .and(warp::post())
        .and(with_auth(env.clone()))
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(
            |env: Environment, user: AdminUser, req: AdminLoginRequest| async move {
                handlers::admin::create_user_handler(env, user, req)
                    .await
                    .map_err(problem::build)
            },
        )
}

fn admin_current_user(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin" / "api" / "v1" / "user")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(env.clone()))
        .and(warp::header::<String>("authorization"))
        .and_then(|env, user, jwt| async move {
            handlers::admin::get_current_user_handler(env, user, jwt)
                .await
                .map_err(problem::build)
        })
}

fn admin_update_password(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("admin" / "api" / "v1" / "password")
        .and(warp::put())
        .and(with_auth(env.clone()))
        .and(warp::header::<String>("authorization"))
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .and_then(
            |env: Environment, user: AdminUser, jwt, req: UpdatePassword| async move {
                handlers::admin::update_password_handler(env, user, jwt, req)
                    .await
                    .map_err(problem::build)
            },
        )
}

fn admin_cosmetics(
    env: Environment,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // cosmetics api
    let prefix = warp::path!("admin" / "api" / "v1" / "cosmetics" / ..);

    // brand

    // GET /../brands
    let create_brands = warp::path!("brands")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_auth(env.clone()))
        .and(warp::body::content_length_limit(10240))
        .and(warp::body::json())
        .and_then(
            |env: Environment, user: AdminUser, brands: Vec<NewBrand>| async move {
                handlers::cosmetics::create_brands(env, brands, user.username.as_str())
                    .await
                    .map_err(problem::build)
            },
        );

    // GET /../brands
    let get_brands = warp::path!("brands")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(env.clone()))
        .and_then(|env: Environment, _user: AdminUser| async move {
            handlers::cosmetics::get_all_brands(env)
                .await
                .map_err(problem::build)
        });

    // PUT /../brands/sequence
    let update_brands_sequence = warp::path!("brands" / "sequence")
        .and(warp::path::end())
        .and(warp::put())
        .and(with_auth(env.clone()))
        .and(warp::body::content_length_limit(10240))
        .and(warp::body::json())
        .and_then(
            |env: Environment, user: AdminUser, bss: Vec<BrandSequence>| async move {
                handlers::cosmetics::update_brands_sequence(env, bss, user.username.as_str())
                    .await
                    .map_err(problem::build)
            },
        );

    // DELETE /../brand/{id}
    let delete_brand = warp::path!("brand" / u32)
        .and(warp::delete())
        .and(with_auth(env.clone()))
        .and_then(|id: u32, env: Environment, user: AdminUser| async move {
            handlers::cosmetics::delete_brand(env, id, user.username.as_str())
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
        .and(with_auth(env.clone()))
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and_then(
            |env: Environment, user: AdminUser, product: NewProduct| async move {
                handlers::cosmetics::create_product(env, product, user.username.as_str())
                    .await
                    .map_err(problem::build)
            },
        );

    // GET /../products
    let get_product_list = warp::path!("products")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(env.clone()))
        .and(warp::query::<Paging>())
        .and_then(
            |env: Environment, _user: AdminUser, paging: Paging| async move {
                handlers::cosmetics::get_products(env, paging)
                    .await
                    .map_err(problem::build)
            },
        );

    // GET /../product/{id}
    let get_product = warp::path!("product" / u32)
        .and(warp::get())
        .and(with_auth(env.clone()))
        .and_then(|id: u32, env: Environment, _user: AdminUser| async move {
            handlers::cosmetics::get_product(env, id)
                .await
                .map_err(problem::build)
        });

    // PUT /../product/{id}
    let update_product = warp::path!("product" / u32)
        .and(warp::put())
        .and(with_auth(env.clone()))
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and_then(
            |id: u32, env: Environment, user: AdminUser, product: NewProduct| async move {
                handlers::cosmetics::update_product(env, id, product, user.username.as_str())
                    .await
                    .map_err(problem::build)
            },
        );

    // DELETE /../product/{id}
    let delete_product = warp::path!("product" / u32)
        .and(warp::delete())
        .and(with_auth(env.clone()))
        .and_then(|id: u32, env: Environment, user: AdminUser| async move {
            handlers::cosmetics::delete_product(env, id, user.username.as_str())
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
        .and(with_auth(env.clone()))
        .and(warp::body::content_length_limit(4096))
        .and(warp::body::json())
        .and_then(
            |env: Environment, user: AdminUser, hps: Vec<u32>| async move {
                handlers::cosmetics::add_hot_product(env, hps, user.username.as_str())
                    .await
                    .map_err(problem::build)
            },
        );

    // GET /../product/hot
    let get_hot_products = warp::path!("product" / "hot")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_auth(env.clone()))
        .and_then(|env: Environment, _user: AdminUser| async move {
            handlers::cosmetics::get_hot_products(env)
                .await
                .map_err(problem::build)
        });

    let api_hot_products = add_hot_product.or(get_hot_products);

    prefix.and(api_brands.or(api_products).or(api_hot_products))
}
