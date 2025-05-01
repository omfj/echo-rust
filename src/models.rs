use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostDatabase {
    pub title: String,
    pub posts: Vec<Post>,
}

impl PostDatabase {
    pub fn init() -> Self {
        let posts_toml = include_str!("../rss.toml");
        toml::de::from_str::<Self>(posts_toml).unwrap()
    }

    pub fn find_by_id(&self, id: String) -> Option<Post> {
        self.posts.iter().find(|p| p.id == id).cloned()
    }

    pub fn all(&self) -> &[Post] {
        &self.posts
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub content: String,
}

impl Post {
    pub fn to_item(&self) -> String {
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
