mod models;
mod handlers;
mod routers;

#[tokio::main]
async fn main() {
    let routes = routers::routes();

    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}