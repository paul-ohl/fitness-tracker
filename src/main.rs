
use axum::Router;
use sport_tracker::pages::new_workout_router;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, server()).await.unwrap();
}

fn server() -> Router {
    Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .merge(new_workout_router())
}
