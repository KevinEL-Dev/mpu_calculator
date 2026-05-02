use axum_login::{
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use axum_login::require::{Require, RedirectHandler};
use axum_messages::MessagesManagerLayer;
use sqlx::{
    Pool,
    Sqlite,
    SqlitePool
};
use tower_http::services::ServeDir;
use time::Duration;
use tokio::{signal, task::AbortHandle};
use tower_sessions::cookie::Key;
use tower_sessions_sqlx_store::SqliteStore;


use crate::{
    users::Backend,
    web::{auth,protected},
    database::{database_exists,init_database,get_database_path},
    handlers::AppState
};

// mod views;
// mod database;
// mod handlers;

pub struct App { 
    db: SqlitePool,
}


impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        if !database_exists().expect("Failed to check database exsistence"){
            if let Err(err) = init_database().await{
                println!("{err}");
            };
        }
        let path = get_database_path().unwrap() +  "/ppmc.sqlite3";
        // let state = AppState{
        //     pool
        // };
        let db = Pool::<Sqlite>::connect(&path).await.unwrap();

        Ok(Self { db })
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        
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

        let require_login = Require::<Backend>::builder()
            .unauthenticated(RedirectHandler::new().login_url("/login"))
            .build();

        let backend = Backend::new(self.db);
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

        // follow axum_login format
        let app = protected::router()
            .route_layer(require_login)
            .merge(auth::router())
            .layer(MessagesManagerLayer)
            .layer(auth_layer)
            .nest_service("/static", ServeDir::new("static"))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;

        deletion_task.await??;

        Ok(())
    }

}
async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
