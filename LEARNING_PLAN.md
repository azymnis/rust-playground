# Rust Learning Plan

Welcome to your Rust learning journey! This guide is structured to take you from beginner to proficient in 6 weeks.

## Prerequisites
- Install Rust: https://www.rust-lang.org/tools/install
- Install VS Code with rust-analyzer extension (recommended)
- Verify installation: `rustc --version` and `cargo --version`

## Learning Path Overview

### Phase 1: Fundamentals (Week 1-2)
**Goal**: Master basic syntax and programming concepts in Rust

#### Topics to Cover:
- Variables and mutability
- Data types (integers, floats, char, bool, strings)
- Functions and return values
- Control flow (if/else, loops)
- Comments and documentation

#### Exercises:
1. **Hello World variations** - `exercises/week1/hello_world`
2. **Variables and mutability** - `exercises/week1/variables`
3. **Functions** - `exercises/week1/functions`
4. **Additional practice** - Create new Cargo projects for:
   - Temperature converter - Fahrenheit to Celsius
   - Fibonacci generator - Using loops and recursion
   - Guessing game - From the Rust Book

### Phase 2: Ownership System (Week 2-3)
**Goal**: Understand Rust's unique memory management model

#### Topics to Cover:
- Ownership rules
- Move semantics
- Borrowing and references
- Mutable vs immutable borrows
- Lifetimes basics

#### Exercises:
1. **Ownership basics** - `exercises/week2/ownership_basics`
2. **Borrowing** - `exercises/week2/borrowing`
3. **Slices** - `exercises/week2/slices`
4. **Ownership with functions** - `exercises/week2/ownership_functions`
5. **Lifetimes introduction** - `exercises/week2/lifetimes_intro`
6. **Practical ownership** - `exercises/week2/practical_ownership`

### Phase 3: Type System (Week 3-4)
**Goal**: Master Rust's powerful type system

#### Topics to Cover:
- Structs and tuple structs
- Enums and pattern matching
- Option<T> and Result<T, E>
- Methods and associated functions
- Type aliases

#### Exercises:
1. **Shape calculator** - Using structs and methods
2. **State machine** - Using enums
3. **Error handling** - Result type practice
4. **Mini database** - Combining concepts

### Phase 4: Advanced Concepts (Week 4-5)
**Goal**: Learn intermediate Rust features

#### Topics to Cover:
- Traits and trait bounds
- Generics
- Iterator trait and closures
- Smart pointers (Box, Rc, RefCell)
- Modules and crates

#### Exercises:
1. **Generic data structures** - Implement Stack/Queue
2. **Custom iterators** - Create your own
3. **Trait implementations** - Display, Debug, etc.
4. **Module organization** - Multi-file project

### Phase 5: Practical Programming (Week 5-6)
**Goal**: Apply knowledge to real projects

#### Topics to Cover:
- Error handling patterns
- Testing (unit and integration)
- Documentation
- Common crates (serde, clap, tokio)
- Async programming basics

#### Projects:
1. **CLI Application** - `projects/cli-app` - Build a file organizer or todo list
2. **Web API** - `projects/web-api` - Create a simple REST service
3. **Systems programming** - Create a new project for file watcher or process monitor

## Resources

### Primary Resources:
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings/)

### Additional Resources:
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Comprehensive Rust (Google's Course)](https://google.github.io/comprehensive-rust/)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

## Tips for Success

1. **Write code daily** - Even 30 minutes helps
2. **Read compiler errors** - Rust's errors are very helpful
3. **Use the playground** - https://play.rust-lang.org/
4. **Join the community** - Rust Discord/Reddit
5. **Don't skip ownership** - It's fundamental to Rust

## Project Structure

```
rust-playground/
├── LEARNING_PLAN.md (this file)
├── README.md
├── exercises/
│   ├── week1/
│   ├── week2/
│   ├── week3/
│   ├── week4/
│   └── week5/
├── projects/
│   ├── cli-app/
│   └── web-api/
└── notes/
    └── concepts.md
```

## Getting Started

1. Install Rust using rustup
2. Run `rustc --version` and `cargo --version` to verify installation
3. From the root directory, build all projects: `cargo build --workspace`
4. Start with week1 exercises:
   ```bash
   cd exercises/week1/hello_world
   cargo run
   ```
5. Follow along with The Rust Book
6. Commit your progress regularly!

## Running Exercises

Each exercise is a Cargo project. To work on an exercise:

```bash
# Navigate to the exercise
cd exercises/week1/hello_world

# Run the exercise
cargo run

# Check for compilation errors
cargo check

# Run tests (if any)
cargo test
```

You can also run from the root directory:

```bash
# Run a specific exercise from root
cargo run -p hello_world
cargo run -p ownership_basics
```

Happy learning! 🦀