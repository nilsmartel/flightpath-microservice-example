use warp::Filter;

mod handler;
mod logic;
mod model;

#[tokio::main]
async fn main() {
    let calculate = warp::get()
        .and(warp::path("calculate"))
        .and(warp::path::end())
        .and(warp::body::json())
        .map(handler::calculate);

    let port = 8080;

    log::info!("serving on port {port}");

    warp::serve(calculate).run(([127, 0, 0, 1], port)).await;
}
