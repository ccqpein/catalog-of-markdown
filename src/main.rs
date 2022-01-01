use catalog_of_markdown::handle_file;
use std::env;

fn main() -> std::io::Result<()> {
    let filepath = env::args().nth(1).unwrap();
    println!("{}", handle_file(&filepath)?.join("\n"));
    Ok(())
}
