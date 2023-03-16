// TODO:
// 1. combine diff_chars, diff_words and diff_lines
// Final goal: function like git-diff

use differ::Differ;
use std::{
    env::{self, Args},
    process::{exit, ExitCode},
};

mod differ;
fn usage(program: &str) {
    eprintln!("Usage: {program} [SBUCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("     diff-words <old_file> <new_file>         find the difference by words between the files.");
    eprintln!("     diff-chars <old_file> <new_file>         find the difference by chars between the files.");
}

fn read_files(args: &mut Args, program: &str) -> Result<(String, String), ()> {
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

    return Ok((old_strings, new_strings));
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program doesn't be provided.");
    let mut subcommand: Option<String> = None;
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "help" | "h" => {
                usage(&program);
                exit(0);
            }
            _ => subcommand = Some(arg),
        }
    }

    let subcommand = subcommand.ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: no subcommand is provided.");
    })?;

    let (old_strings, new_strings) = read_files(&mut args, &program)?;

    let differ = Differ::new(new_strings, old_strings);

    match subcommand.as_str() {
        "diff-words" => {
            let result = differ.diff_by_words();

            println!("diff result: {:?}", result);
        }

        "diff-chars" => {
            let result = differ.diff_by_chars();

            println!("diff result: {:?}", result);
        }
        _ => {
            eprintln!("ERROR: unknown subcommand {subcommand}");
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
