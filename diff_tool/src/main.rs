// TODO:
// 1. combine diff_chars, diff_words and diff_lines
// 2. separate functions or structs into different directories
// Final goal: function like git-diff

use std::{env, process};

use diff_tool::{compute_lcs_matrix_dp, diff, Config, Content};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Old file path: {}", config.old_file_path);
    println!("New file path: {}", config.new_file_path);

    match Content::read(config) {
        Ok(content) => {
            // NOTE: diff words
            let result = diff(
                &content
                    .new_text
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                &content
                    .old_text
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            );

            println!("diff result: {:?}", result);
        }
        Err(err) => {
            eprintln!("Application Error: {err}.");
        }
    }
}
