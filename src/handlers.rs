use crate::{models::RSSToml, xml::Xml, AppState};

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
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

pub async fn feed() -> Result<Xml<String>, StatusCode> {
    let posts_toml = include_str!("../rss.toml");
    let posts: RSSToml =
        toml::de::from_str(posts_toml).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = r#"<?xml version="1.0" ?>
<rss
    xmlns:dc="http://purl.org/dc/elements/1.1/"
    xmlns:content="http://purl.org/rss/1.0/modules/content/"
    xmlns:atom="http://www.w3.org/2005/Atom"
    version="2.0"
>
    <channel>
        <title><![CDATA[echo::rust(🦀)]]></title>
        <link>https://echo-rust.fly.dev/</link>
        <description>Innlegg på echo::rust(🦀) - Interessegruppen for Rust på echo</description>
        <language>nb</language>
        <lastBuildDate>Wed, 30 Apr 2025 09:00:00 GMT</lastBuildDate>
        <item>
            <title><![CDATA[Første innlegg!]]></title>
            <link>https://echo-rust.fly.dev/innlegg/5b38a73f-28bc-4220-8c1f-30e07a92d4d5</link>
            <guid isPermaLink="false">5b38a73f-28bc-4220-8c1f-30e07a92d4d5</guid>
            <dc:creator><![CDATA[Webkom 💻]]></dc:creator>
            <pubDate>Tue, 08 Apr 2025 09:41:02 GMT</pubDate>
            <description><![CDATA[Velkommen til den nye nettsiden]]></description>
            <content:encoded><![CDATA[Velkommen til den nye nettsiden]]></content:encoded>
        </item>
    </channel>
</rss>
"#;

    Ok(Xml(body.to_string()))
}
