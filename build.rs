use chrono::format::strftime::StrftimeItems;
use chrono::Utc;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Command::new("npm").args(&["run", "css-build"]).spawn()?;

    let now = Utc::now();
    let fmt = StrftimeItems::new("%a, %d %b %Y %H:%M:%S GMT");
    println!("cargo:rustc-env=BUILD_DATE={}", now.format_with_items(fmt));

    Ok(())
}
