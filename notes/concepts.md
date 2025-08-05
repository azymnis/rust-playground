# Rust Concepts Notes

Use this file to keep track of important concepts and "aha!" moments.

## Key Differences from Other Languages

### Coming from C/C++:
- No manual memory management (no malloc/free)
- No null pointers (use Option<T> instead)
- No undefined behavior in safe Rust
- Pattern matching is more powerful

### Coming from Java/Scala:
- No garbage collector
- Immutability by default
- No inheritance (use composition and traits)
- Zero-cost abstractions

### Coming from JavaScript/Node:
- Statically typed
- Compiled language
- No runtime overhead
- Explicit error handling

## Important Concepts to Remember

### Ownership Rules:
1. Each value has a single owner
2. When the owner goes out of scope, the value is dropped
3. There can only be one mutable reference OR multiple immutable references

### Common Compiler Errors:
- "borrowed value does not live long enough"
- "cannot borrow as mutable more than once"
- "use of moved value"

## Useful Commands

```bash
# Format code
rustfmt filename.rs

# Check for errors without building
cargo check

# Run with release optimizations
cargo run --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Personal Notes

_Add your own discoveries and insights here as you learn!_