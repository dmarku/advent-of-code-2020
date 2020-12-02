# Implementations for Advent of Code 2020

All days so far have at least a partial solution in Rust. There's no additional libraries, every program can be built in the respective directory with:

```
$ rustc aoc-2020-<day>.rs
```

- day 1 is completely implemented in Python; only part I is implemented in Rust, too
- day 2 requires Rust nightly (`split_once()` is not available in `std::string` in Rust 1.48)
