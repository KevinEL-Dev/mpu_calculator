use axum::{Router, routing::get};
mod views;
#[tokio::main]
async fn main() {

    let app = Router::new().route("/",get(views::hello_world));

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listner, app.into_make_service()).await.unwrap();
}
