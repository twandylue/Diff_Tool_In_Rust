// TODO:
// 1. combine diff_chars, diff_words and diff_lines
// 2. separate functions or structs into different directories
// Final goal: function like git-diff

use differ::Differ;
use std::env;

mod differ;

fn usage(program: &str) {
    eprintln!("Usage: {program} <old_file> <new_file>");
}

fn main() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program doesn't be provided.");
    let old_file = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no old file path is provided.");
    })?;

    let new_file = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no new file path is provided.");
    })?;

    println!("Old file path: {old_file}");
    println!("New file path: {new_file}");

    let old_strings = std::fs::read_to_string(&old_file).map_err(|err| {
        usage(&program);
        eprintln!("ERROR: could not read {old_file}: {err}");
    })?;

    let new_strings = std::fs::read_to_string(&new_file).map_err(|err| {
        usage(&program);
        eprintln!("ERROR: could not read {new_file}: {err}");
    })?;

    let differ = Differ {
        new_text: new_strings,
        old_text: old_strings,
    };

    let result = differ.diff_by_words();

    println!("diff result: {:?}", result);

    Ok(())
}
