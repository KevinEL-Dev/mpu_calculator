use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use axum_messages::{Message, Messages};

use crate::views::{hello_world};
use crate::users::AuthSession;
use crate::web::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(self::get::protected))
}

mod get {
    // does this just use the libraries in this file?
    use super::*;

    pub async fn protected(auth_session: AuthSession,messages: Messages) -> impl IntoResponse {
        match auth_session.user().await {
            Some(user) => Html(hello_world().await.into_string()).into_response(),
            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
