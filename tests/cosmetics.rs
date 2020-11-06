use kerria::api;
use warp::hyper::StatusCode;

#[tokio::test]
async fn test_status_ok() {
    let req = api::status();
    let resp = warp::test::request()
        .method("GET")
        .path("/status")
        .reply(&req)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
}
