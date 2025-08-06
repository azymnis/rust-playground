# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust learning playground repository structured as a 6-week educational journey from beginner to proficient Rust developer. The repository uses Cargo workspaces to manage all exercises and projects.

## Repository Structure

- `exercises/`: Weekly exercise Cargo projects organized by learning phase
  - `week1/`: Basic syntax (hello_world, variables, functions)
  - `week2/`: Ownership system (ownership_basics, borrowing, slices, ownership_functions, lifetimes_intro, practical_ownership)
  - `week3-5/`: Advanced topics (to be populated)
- `projects/`: Practical application projects
  - `cli-app/`: Command-line application project
  - `web-api/`: REST API project
- `notes/`: Learning notes and concept documentation
- `Cargo.toml`: Workspace configuration managing all sub-projects

## Development Commands

### Running Individual Exercises
```bash
# Navigate to specific exercise
cd exercises/week1/hello_world
cargo run

# Or run from root directory
cargo run -p hello_world
cargo run -p ownership_basics
```

### Common Cargo Commands
```bash
# Check compilation without building
cargo check

# Build the project
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Format code (if rustfmt is installed)
cargo fmt

# Lint code (if clippy is installed)
cargo clippy
```

### Workspace Commands (from root)
```bash
# Build all projects
cargo build --workspace

# Check all projects
cargo check --workspace

# Test all projects
cargo test --workspace

# Format all projects
cargo fmt --workspace

# Lint all projects
cargo clippy --workspace
```

## Creating New Exercises

To add a new exercise:
```bash
# Create new exercise project
cargo new --name exercise_name exercises/week3/exercise_name

# The workspace will automatically include it
```

## Key Learning Path

The repository follows a structured 6-week learning plan:

1. **Week 1-2**: Fundamentals (variables, functions, control flow)
2. **Week 2-3**: Ownership System (borrowing, lifetimes)
3. **Week 3-4**: Type System (structs, enums, error handling)
4. **Week 4-5**: Advanced Concepts (traits, generics, iterators)
5. **Week 5-6**: Practical Programming (real projects)

## Exercise Guidelines

- Each exercise is a standalone Cargo project with its own `Cargo.toml`
- Exercises contain TODO comments indicating tasks to complete
- All exercises can be run with `cargo run` from their respective directories
- Exercises build upon each other - ensure prerequisite concepts are understood
- The repository is set up for learning, not production - prioritize clarity over optimization

## Working with Dependencies

Dependencies can be added at two levels:

1. **Workspace-level** (in root `Cargo.toml` under `[workspace.dependencies]`):
   - Shared across all projects
   - Use `dependency.workspace = true` in project `Cargo.toml`

2. **Project-level** (in individual project `Cargo.toml`):
   - Specific to that project only

## Common Issues and Solutions

- **Compilation errors in exercises**: Many exercises intentionally have errors for learning. Read TODO comments for guidance.
- **Workspace member not found**: Ensure the project is listed in the root `Cargo.toml` members array
- **Dependency conflicts**: Check if using workspace vs project-level dependencies correctly

## Notes for Future Development

- When helping with exercises, check the specific week's learning goals
- Use `cargo check` frequently for fast feedback on compilation errors
- The workspace structure allows running all tests at once with `cargo test --workspace`
- When creating new projects, use descriptive names that reflect the learning objective