# Diff Tool In Rust

Find the difference between two documents (like the function of git diff).

## Quick Start

Prepare two different files to compare the difference between them.

* In `old_text.txt`

  ```plain
  I am Andy. Here I come.
  ```

* In `new_text.txt`

  ```plain
  I am Amy.
  ```

Then, let's go!

```console
$ cargo run -- old_file.txt new_file.txt
Old file path: old_text.txt
New file path: new_text.txt
diff result: ["I", "am", "-Andy.", "-Here", "-I", "-come.\n", "+Amy.\n"]
```

## Algorithm

LCS(Longest Common Sequence)

## TODOs

* [ ] Using webassembly(maybe [Yew](https://github.com/yewstack/yew)) to display the diff-result.

## References

* [diffing](https://florian.github.io/diffing/)
