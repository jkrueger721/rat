fn main() {
    let path: std::path::PathBuf = std::env::args().nth(1).expect("no input provided").into();
    let content = std::fs::read_to_string(&path).expect("could not read file");
    for line in content.lines() {
        println!("{}", line);
    }
}
