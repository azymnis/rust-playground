# Week 2: Ownership, Borrowing, and Lifetimes

This week covers Rust's most unique feature - the ownership system. This is what enables memory safety without a garbage collector.

## Core Concepts

### Ownership Rules
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

### Borrowing Rules
1. You can have either one mutable reference OR any number of immutable references
2. References must always be valid
3. The borrow checker enforces these rules at compile time

## Exercises

1. **01_ownership_basics.rs** - Understanding move semantics
2. **02_borrowing.rs** - Immutable and mutable references
3. **03_slices.rs** - Working with string and array slices
4. **04_ownership_functions.rs** - Ownership in function parameters
5. **05_lifetimes_intro.rs** - Introduction to lifetime annotations
6. **06_practical_ownership.rs** - Real-world ownership scenarios

## Key Concepts to Master

- [ ] Move vs Copy semantics
- [ ] Stack vs Heap allocation
- [ ] Borrowing with & and &mut
- [ ] The borrow checker
- [ ] String vs &str
- [ ] Vector ownership
- [ ] Lifetime elision rules

## Common Errors You'll See

- "value borrowed here after move"
- "cannot borrow as mutable more than once"
- "borrowed value does not live long enough"

Don't worry! These errors are Rust protecting you from memory bugs.

## Tips

1. When you get a borrow checker error, read it carefully - it tells you exactly what's wrong
2. Drawing ownership diagrams can help visualize what's happening
3. Start with immutable borrows (&) before trying mutable ones (&mut)
4. Remember: Rust is preventing real bugs that would crash in C/C++

## Resources

- [Chapter 4 of The Rust Book](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Ownership by Example](https://doc.rust-lang.org/rust-by-example/scope.html)
- [Visualizing Ownership](https://rufflewind.com/img/rust-move-copy-borrow.png)