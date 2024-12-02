# Advent of Code
Solutions for Advent of Code, with my puzzle data included. Run it like this, with year and day as command line arguments:
```
cargo run -- 2024 2
```
Output:
```
   Compiling aoc v0.1.0 (/home/frank/data/projects/aoc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/aoc 2024 2`
solution 1: 510
solution 2: 553
```
Regression tests for all days:
```
$ cargo test
   Compiling aoc v0.1.0 (/home/frank/data/projects/aoc)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running unittests src/main.rs (target/debug/deps/aoc-f248d5a3bf4e7bb6)

running 1 test
test tests::test2024 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
