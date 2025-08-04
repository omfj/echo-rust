use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOMLPost {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TOMLConfig {
    pub title: String,
    pub posts: Vec<TOMLPost>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub content: String,
}

impl From<TOMLPost> for Post {
    fn from(post: TOMLPost) -> Self {
        let date = DateTime::parse_from_rfc2822(&post.date)
            .expect("Failed to parse date")
            .with_timezone(&Utc);
        Post {
            id: post.id,
            title: post.title,
            description: post.description,
            date,
            content: post.content,
        }
    }
}

impl Post {
    pub fn to_rss_item(&self) -> String {
        format!(
            r#"<item>
            <title><![CDATA[{}]]></title>
            <link>https://echo-rust.fly.dev/post/{}</link>
            <guid isPermaLink="false">{}</guid>
            <dc:creator><![CDATA[echo::rust(🦀)]]></dc:creator>
            <pubDate>{}</pubDate>
            <content:encoded><![CDATA[{}]]></content:encoded>
        </item>"#,
            self.title, self.id, self.id, self.date, self.content
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RSSImporter {
    pub title: String,
    pub posts: Vec<Post>,
}

impl RSSImporter {
    pub fn init() -> Self {
        let posts_rss = include_str!("../rss.toml");
        let config: TOMLConfig = toml::from_str(posts_rss).expect("Failed to parse RSS TOML");
        let posts: Vec<Post> = config.posts.into_iter().map(Post::from).collect();
        RSSImporter {
            title: config.title,
            posts,
        }
    }

    pub fn find_by_id(&self, id: String) -> Option<Post> {
        self.posts.iter().find(|p| p.id == id).cloned()
    }

    pub fn all(&self) -> &[Post] {
        &self.posts
    }
}
