# Diff Tool In Rust

Find and show the difference between two documents (like the function of git diff).

## Quick Start

Prepare two different files(`*.txt`) to compare the difference between them.

In `old.txt`

```plain
I am Andy. Here I come.
```

In `new.txt`

```plain
I am Amy.
```

Then, let's go!

```console
$ cargo run -- old.txt new.txt
Old file path: old.txt
New file path: new.txt
diff result: ["I", "am", "-Andy.", "-Here", "-I", "-come.\n", "+Amy.\n"]
```

## Algorithm

LCS(Longest Common Sequence)

## TODOs

* [ ] UI(Using webassembly(maybe [Yew](https://github.com/yewstack/yew)) to display the diff-result.)

## References

* [diffing](https://florian.github.io/diffing/)
