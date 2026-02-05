use axum::{Router, response::Redirect, routing::get};
use tower_http::services::ServeDir;

use crate::inbound::frontend_routes::new_workout::new_workout_page;

pub fn server() -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .nest("/api", backend_routes())
        .merge(frontend_routes())
}

pub fn backend_routes() -> Router {
    Router::new()
}

pub fn frontend_routes() -> Router {
    Router::new().route("/new", get(new_workout_page).post(Redirect::to("/new")))
}
