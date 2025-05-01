use crate::{models::Post, xml::Xml, AppState};

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    Json,
};
use minijinja::context;

pub async fn root(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();
    let template = template.render(context! {}).unwrap();

    Ok(Html(template))
}

pub async fn fallback(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("error").unwrap();
    let template = template.render(context! {}).unwrap();

    Ok(Html(template))
}

pub async fn single_post(
    Path(post_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Post>, StatusCode> {
    let post = state.post_db.find_by_id(post_id);

    match post {
        Some(post) => Ok(Json(post)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn feed(State(state): State<Arc<AppState>>) -> Result<Xml<String>, StatusCode> {
    let items = state
        .post_db
        .all()
        .iter()
        .map(|p| p.to_item())
        .collect::<Vec<_>>()
        .join("\n");

    let body = format!(
        r#"<?xml version="1.0" ?>
<rss
    xmlns:dc="http://purl.org/dc/elements/1.1/"
    xmlns:content="http://purl.org/rss/1.0/modules/content/"
    xmlns:atom="http://www.w3.org/2005/Atom"
    version="2.0"
>
    <channel>
        <title><![CDATA[{}]]></title>
        <link>https://echo-rust.fly.dev/</link>
        <description>Innlegg på echo::rust(🦀) - Interessegruppen for Rust på echo</description>
        <language>nb</language>
        <lastBuildDate>{}</lastBuildDate>
    {}
    </channel>
</rss>
"#,
        state.post_db.title, state.last_build_date, items
    );

    Ok(Xml(body))
}
