use catalog_of_markdown::{handle_file, Args};
use clap::Parser;

fn main() -> std::io::Result<()> {
    let commandline_args = Args::parse();

    println!(
        "{}",
        handle_file(&commandline_args.md_file, commandline_args.depth)?.join("\n")
    );
    Ok(())
}
