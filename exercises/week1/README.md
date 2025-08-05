# Week 1: Rust Fundamentals

This week covers the basic building blocks of Rust programming.

## Exercises

1. **01_hello_world.rs** - Your first Rust program
2. **02_variables.rs** - Variables, mutability, and types
3. **03_functions.rs** - Functions and return values
4. **04_control_flow.rs** - if/else and loops
5. **05_temperature_converter.rs** - Practice project

## How to Run

For individual files:
```bash
rustc filename.rs
./filename
```

Or use cargo (after creating a project):
```bash
cargo new my_project
cd my_project
# Copy exercise code into src/main.rs
cargo run
```

## Learning Goals

By the end of this week, you should understand:
- [ ] How to declare variables and constants
- [ ] The difference between mutable and immutable
- [ ] Basic data types
- [ ] How to write and call functions
- [ ] Control flow with if/else
- [ ] Different types of loops

## Tips

1. Read compiler error messages carefully - they're very helpful!
2. Use `cargo check` to quickly check for errors without building
3. The Rust Playground (https://play.rust-lang.org/) is great for quick experiments

## Next Steps

After completing these exercises, try the guessing game from Chapter 2 of The Rust Book!