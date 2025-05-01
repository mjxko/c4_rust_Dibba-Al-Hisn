# C4 Compiler Rewritten in Rust

## Objective:
Building on your analysis of the C4 compiler from Assignment 1, your team will rewrite the
C4 compiler in Rust. The Rust version must compile the same subset of C code as the
original C4 compiler (e.g., the C4 source code itself), maintaining its self-hosting capability
and core functionality. This project leverages Rust’s safety, modern features, and
performance to reimplement the compiler, ensuring equivalence to the original while
improving design where possible.

## How to Build & Run: 
- Rust (Install via [https://rust-lang.org](https://www.rust-lang.org/learn/get-started) )

##  How to Run
```bash
cargo build
cargo run --path/to/source.c
```

## Tests
Run all tests:
```bash
cargo test
```


## The bonus Feature is an enhanced error reporting
this Rust version improves on out original C4 compiler by including a more precise syntax error reporting, so when an invalid statement or unexpected token is encountered then the compiler will output a detailed message such as:

```
Syntax Error: expected ';' after printf() at line 2, col 18
```

it makes debugging much easier compared to our original c4 version



## done by:
- Mahra Almazrouei
- Sarah Alkaabi
