use chrono::{format::StrftimeItems, DateTime, Utc};
use std::io::Write;
use uuid::Uuid;

fn main() -> anyhow::Result<(), std::io::Error> {
    let id = Uuid::new_v4().to_string();
    let date = get_date_2822();

    let mut rss_file = std::fs::OpenOptions::new().append(true).open("rss.toml")?;

    writeln!(rss_file, "")?;
    writeln!(rss_file, "[[posts]]")?;
    writeln!(rss_file, "id = \"{}\"", id)?;
    writeln!(rss_file, "title = \"\"")?;
    writeln!(rss_file, "description = \"\"")?;
    writeln!(rss_file, "date = \"{}\"", date)?;
    writeln!(rss_file, "content = \"\"\"")?;
    writeln!(rss_file, "\"\"\"")?;

    Ok(())
}

fn get_date_2822() -> String {
    let now: DateTime<Utc> = Utc::now();
    let rfc2822_format = StrftimeItems::new("%a, %d %b %Y %H:%M:%S GMT");
    now.format_with_items(rfc2822_format).to_string()
}
