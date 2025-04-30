use crate::handlers::{fallback, feed, root};

use axum::{routing::get, Router};
use minijinja::Environment;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod xml;

pub struct AppState {
    pub env: Environment<'static>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let env = create_env();
    let serve_public = ServeDir::new("static");
    let app_state = Arc::new(AppState { env });

    let app = Router::new()
        .route("/", get(root))
        .route("/feed", get(feed))
        .nest_service("/static", serve_public)
        .fallback(fallback)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn create_env() -> Environment<'static> {
    let mut env = Environment::new();

    let templates = [
        (
            "components/base-layout",
            include_str!("../templates/components/base-layout.html.j2"),
        ),
        ("home", include_str!("../templates/home.html.j2")),
        ("error", include_str!("../templates/error.html.j2")),
    ];

    for (name, template) in templates {
        env.add_template(name, template).unwrap();
    }

    env
}
