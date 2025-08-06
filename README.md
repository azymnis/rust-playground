# rust-playground
Learning rust for fun

## Quick Start

This repository uses Cargo workspaces to manage all exercises and projects.

### Prerequisites
- Install Rust: https://www.rust-lang.org/tools/install
- Verify installation: `rustc --version` and `cargo --version`

### Running Exercises

All exercises are organized as Cargo projects. To run any exercise:

```bash
# Navigate to the exercise directory
cd exercises/week1/hello_world

# Run the exercise
cargo run

# Check for compilation errors
cargo check

# Build without running
cargo build
```

### Workspace Commands

From the root directory, you can run workspace-wide commands:

```bash
# Build all projects
cargo build --workspace

# Run tests for all projects
cargo test --workspace

# Check all projects
cargo check --workspace

# Run a specific project from root
cargo run -p hello_world
cargo run -p ownership_basics
```

### Project Structure

- `exercises/`: Weekly exercise Cargo projects
  - `week1/`: Basic syntax exercises (hello_world, variables, functions)
  - `week2/`: Ownership system exercises (ownership_basics, borrowing, slices, etc.)
- `projects/`: Full application projects
  - `cli-app/`: Command-line application project
  - `web-api/`: REST API project
- `notes/`: Learning notes and documentation

See [LEARNING_PLAN.md](LEARNING_PLAN.md) for the complete 6-week learning curriculum.
