use axum::{
    Router,
    routing::{get,post}
};
use sqlx::{
    Pool,
    Sqlite
};
mod views;
mod database;
mod handlers;
#[tokio::main]
async fn main() {

    if !database::database_exists().expect("Failed to check database exsistence"){
        // create the database
        println!("creating database...");
        if let Err(err) = database::init_database().await{
            println!("{err}");
        };
    }
    let path = database::get_database_path().unwrap() +  "/ppmc.sqlite3";
    let pool = Pool::<Sqlite>::connect(&path).await.unwrap();
    let state = handlers::AppState{
        pool
    };
    let app = Router::new()
        .route("/",get(views::hello_world))
        .route("/create_source",post(handlers::create_source))
        .route("/create_measurement_unit",post(handlers::create_measurement_unit))
        .with_state(state);

    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listner, app.into_make_service()).await.unwrap();
}
