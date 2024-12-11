# Advent Of Code 2024

## Purpose
Solutions for aoc 2024 written in rust.

## Lib Structure
```
├── Cargo.lock
├── Cargo.toml                  -> project config including dependencies
├── data                        -> users custom input, ignored by github [please dont share your user input!](https://adventofcode.com/2023/about)
│   └── day{}.txt
├── makeday.sh                  -> generates boilerplate code for each day so it can be used in the module_runner
├── README.md                   -> docs.. you are here!
├── src                         -> uncompiled raw rust code
│   ├── days
│   │   ├── day{}.rs            -> sepperate files for each day
│   │   └── mod.rs              -> loads file for each day into the days module
│   ├── error.rs                -> custom error classes for the project
│   ├── lib.rs                  -> modules that are available including days and each day submodule, utils, and the module_runner
│   ├── main.rs                 -> parses command line arguments: usage: `cargo run $DAY $PART`
│   ├── module_runner.rs        -> parses the day and part args into a module runner which will run the solution for the relevant day and part from the days submodule
│   └── template.rs             -> boilerplate code for each day so it can be correctly found by the module_runner::SolutionRunner
│   └── utils.rs                -> common functions that may be used across days
├── target                      -> compiled binaries, ignored by github
└── tests
    └── integration_test.rs     -> completed solutions can be revalidated provide the input in data/day{x}.txt and your solution answer in the test_case
```


## Testing
unittests in each day deal with the test inputs
integration tests are created using the test-case crate in tests/integration_tests.rs
integration tests use the module_runner::SolutionRunner to validate the users unique input in data following a correct solution
integration tests support the ability to refactor the code base safely understanding if there are any breaking changes in the functionality following refactoring or sharing utilities across different days.
It also provides ability to benchmark code where performance upgrades would be most needed.
