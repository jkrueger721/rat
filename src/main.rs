fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = std::env::args().nth(1).expect("no input provided").into();
    let content = std::fs::read_to_string(&path)?;
    println!("{}", content);
    Ok(())
}
