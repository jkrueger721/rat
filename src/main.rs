use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path: std::path::PathBuf = std::env::args().nth(1).expect("no input provided").into();
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;
    println!("{}", content);
    Ok(())
}
