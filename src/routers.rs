use warp::Filter;
use super::handlers;

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_post()
}

pub fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("post" / u64)
        .and(warp::get())
        .and_then(handlers::get_post)
}