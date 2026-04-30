use axum_login:: {
    require::{RedirectHandler, Require},
    tower_sessions::{ExpireDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use axum_messages::MessagesManagerLayer;
use sqlx::SqlitePool;
use time::Duration;
use tokio::{signal, task::AbortHandle};
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::SqliteStore;


use crate::{
    users::Backend,
    web::{auth,protected},
};

mod views; 
mod handlers;

pub struct App { 
    db: SqlitePool,
}


impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        if !database::database_exists().expect("Failed to check database exsistence"){
            if let Err(err) = database::init_database().await{
                println!("{err}");
            };
        }
        let path = database::get_database_path().unwrap() +  "/ppmc.sqlite3";
        let state = handlers::AppState{
            pool
        };
        let db = Pool::<Sqlite>::connect(&path).await.unwrap();
        sqlx::migrate!().run(&db).await?;

        Ok(Self { db })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::errorr::Error>> {
        
        let session_store = SqliteStore::new(self.db.clone());
        session_store.migrate().await?;

        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        // generates cryptographic key to sign the session cookie, think about creating env for
        // this
        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        let require_login = Require<Backend>::builder()
            .unauthenticated(RedirectHandler::new().login_url("/login"))
            .build();

        // follow axum_login format
        let app = Router::new()
            .nest_service("/static", ServeDir::new("static"))
            .route("/",get(views::hello_world))
            .route("/login",get(views::login))
            .route("/create_source",post(handlers::create_source))
            .route("/create_measurement_unit",post(handlers::create_measurement_unit))
            .route("/create_meal",post(handlers::create_meal))
            .route("/create_ingredient",post(handlers::create_ingredient))
            .route("/search_sources",get(handlers::search_sources))
            .route("/search_measurement_units",get(handlers::search_measurement_units))
            .route("/search_meals",get(handlers::search_meals))
            .route("/register",get(views::get_register).post(handlers::register))
            .with_state(state);

    }
}
