# rust-wasm

- https://doc.rust-lang.org/book
- vscode

# Cargo - package manager

1. cargo
2. rustc: 컴파일러
3. rustup

```bash
cargo new example or cargo init
```

# Hello world

```bash
cargo build
cargo run
```

# Memory Management

## The Stack

- it's special region of the process memory that stores variables created by each function.
- for every function call a new stack frame is allocated on top of the current on.
- the size of every variable on the stack has to be known at complier time.
- when a function exits it's stack frame is released.

## The Heap

## Pointers

## Smart Pointers
