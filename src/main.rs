use crate::handlers::{fallback, feed, root};
use crate::models::PostDatabase;

use axum::{routing::get, Router};
use handlers::single_post;
use minijinja::Environment;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;
mod xml;

pub struct AppState {
    pub post_db: PostDatabase,
    pub env: Environment<'static>,
    pub last_build_date: String,
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

    let last_build_date = env!("BUILD_DATE").to_string();
    let post_db = PostDatabase::init();
    let env = create_env();
    let serve_public = ServeDir::new("static");
    let app_state = Arc::new(AppState {
        env,
        post_db,
        last_build_date,
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/feed", get(feed))
        .route("/post/{id}", get(single_post))
        .nest_service("/static", serve_public)
        .fallback(fallback)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
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
