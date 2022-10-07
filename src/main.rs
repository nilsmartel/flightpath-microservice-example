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

    warp::serve(calculate).run(([127, 0, 0, 1], 8080)).await;
}
