mod regex_handler;

use clap::Parser;
use regex_handler::*;
use std::io::prelude::*;
use std::io::Result;

/// Command Line Args
#[derive(Default, Parser, Debug)]
#[command(author = "ccQpein", version, about)]
pub struct Args {
    #[arg(short, long, default_value = "0")]
    pub depth: usize,

    #[arg(value_name = "file")]
    pub md_file: String,

    //:= when it is the number, should be a bit different
    #[arg(short, long, default_value = "-")]
    pub sym: String,
}

/// handle the makedown file
pub fn handle_file(filepath: &str, depth: usize, list_sym: &str) -> Result<Vec<String>> {
    let mut buf = vec![];
    let mut f: std::fs::File = std::fs::File::open(filepath)?;
    f.read_to_end(&mut buf)?;
    let mut buf = buf.as_slice();

    let mut result = vec![];
    let mut cache = String::new();
    let mut escaped = false;
    loop {
        cache.clear();
        match buf.read_line(&mut cache) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                if let Some(ccache) = clean_line_content(&cache) {
                    escaped ^= check_if_escape(ccache);
                    if escaped {
                        continue;
                    }
                    match line_handler(ccache, &mut result, depth, list_sym) {
                        Ok(_) => (),
                        Err(_s) => (), //return Err(Error::new(ErrorKind::InvalidData, s)),
                    };
                } else {
                    continue;
                }
            }
        }
    }

    Ok(result)
}

fn clean_line_content(s: &str) -> Option<&str> {
    let ss = s.trim_start_matches(['\n', ' ']);
    if ss.len() == 0 {
        None
    } else {
        Some(ss)
    }
}

fn line_handler(
    s: &str,
    bucket: &mut Vec<String>,
    depth: usize,
    list_sym: &str,
) -> std::result::Result<(), String> {
    match capture_title(s) {
        Some(cap) => {
            let m = pick_the_head(&cap)?;
            let offset = head_count(&m) - 1;
            let space_len = if offset >= depth {
                offset - depth
            } else {
                return Ok(());
            };
            let content = pick_the_head_content(&cap)?;
            let content = content.trim_end_matches([' ', '\n']);
            //:= the format here
            let line = format!(
                "{}{} [{}](#{})",
                std::iter::repeat("  ").take(space_len).collect::<String>(),
                list_sym,
                content,
                String::from_iter(
                    content
                        .to_lowercase()
                        .chars()
                        .filter(|&c| c.is_alphanumeric() || c.is_whitespace() || c == '-')
                )
                .split(" ")
                .collect::<Vec<_>>()
                .join("-")
            );
            bucket.push(line);
            Ok(())
        }
        None => Err(format!("capture title failed: {}", s)),
    }
}

fn check_if_escape(s: &str) -> bool {
    s.starts_with("```")
}

#[cfg(test)]
mod tests {
    use crate::{check_if_escape, clean_line_content, line_handler};

    #[test]
    fn test_line_handler() -> Result<(), String> {
        let mut bucket = vec![];

        let case = "## level 2 ##";
        line_handler(case, &mut bucket, 0, "-")?;
        assert_eq!(bucket[0], "  - [level 2](#level-2)".to_string());

        let case = "# Level 1  ";
        line_handler(case, &mut bucket, 0, "-")?;
        assert_eq!(bucket[1], "- [Level 1](#level-1)".to_string());

        let case = "# Level 1  ";
        line_handler(case, &mut bucket, 0, "*")?;
        assert_eq!(bucket[2], "* [Level 1](#level-1)".to_string());

        Ok(())
    }

    #[test]
    fn test_line_handler_special_char() -> Result<(), String> {
        let mut bucket = vec![];

        let case = clean_line_content(" ## level ,2 ##").unwrap();
        line_handler(case, &mut bucket, 0, "-")?;
        assert_eq!(bucket[0], "  - [level ,2](#level-2)".to_string());

        let case = "# level 1 & c ";
        line_handler(case, &mut bucket, 0, "-")?;
        assert_eq!(bucket[1], "- [level 1 & c](#level-1--c)".to_string());

        let case = "## with-between words ##";

        line_handler(case, &mut bucket, 0, "-")?;
        assert_eq!(
            bucket[2],
            "  - [with-between words](#with-between-words)".to_string()
        );
        Ok(())
    }

    #[test]
    fn test_escape_logic() {
        let mut escape = false;

        escape ^= false;
        assert!(!escape);

        escape ^= true;
        assert!(escape);

        escape ^= false;
        assert!(escape);

        escape ^= true;
        assert!(!escape);
    }

    #[test]
    fn test_check_if_escape() {
        let s = String::from("```rust");
        assert!(check_if_escape(&s));

        assert!(check_if_escape("```"));
    }
}
