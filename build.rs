use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Command::new("npm").args(&["run", "css-build"]).spawn()?;

    Ok(())
}
