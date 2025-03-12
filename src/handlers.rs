use super::models::Post;

pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Post {
        id,
        title: String::from("hello world"),
        body: String::from("hello world hehehe"),
    };
    Ok(warp::reply::json(&post))
}