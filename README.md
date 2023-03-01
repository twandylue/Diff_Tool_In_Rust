# Diff Tool In Rust

Find and show the difference between two documents (like the function of git diff).

## Quick Start

Prepare two different files(`*.txt`) to compare the difference between them.

For help

```console
$ cargo run -- help
Usage: target/debug/diff_tool [SBUCOMMAND] [OPTIONS]
Subcommands:
     diff-words <old_file> <new_file>          find the difference by words between the files.
     diff-chars <old_file> <new_file>          find the difference by chars between the files.
```

In `old.txt`

```plain
I am Andy. Here I come.
```

In `new.txt`

```plain
I am Amy.
```

Then, let's go!

Diff by words

```console
$ cargo run -- diff-words ./src/data/old.txt ./src/data/new.txt
Old file path: ./src/data/old.txt
New file path: ./src/data/new.txt
diff result: ["I", "am", "-Andy.", "-Here", "-I", "-come.\r\n", "+Amy.\r\n"]
```

Diff by chars

```console
$ cargo run -- diff-chars ./src/data/old.txt ./src/data/new.txt
Old file path: ./src/data/old.txt
New file path: ./src/data/new.txt
diff result: ["I", " ", "a", "m", " ", "A", "-n", "-d", "-y", "-.", "- ", "-H", "-e", "-r", "-e", "- ", "-I", "- ", "-c", "-o", 
"m", "-e", "+y", ".", "\r", "\n"]
```

## Algorithm

LCS(Longest Common Sequence)

## TODOs

* [ ] UI(Using webassembly(maybe [Yew](https://github.com/yewstack/yew)) to display the diff-result.)

## References

* [diffing](https://florian.github.io/diffing/)
