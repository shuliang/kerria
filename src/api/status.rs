use warp::filters::BoxedFilter;
use warp::Filter;

fn status1() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and(warp::path::end())
        .map(|| format!("STATUS OK"))
}

fn status2() -> BoxedFilter<(String,)> {
    warp::path!("status" / "2")
        .and(warp::get())
        .and(warp::path::end())
        .map(|| format!("STATUS 2 IS OK."))
        .boxed()
}

pub fn status() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    status1().or(status2())
}
