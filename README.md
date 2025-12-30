# Ruje

A Clojure implementation written in Rust.

## Overview

Ruje is an experimental Clojure interpreter built from scratch in Rust. The project aims to implement core Clojure semantics while leveraging Rust's performance and safety guarantees.

## Features

- Interactive REPL for evaluating Clojure expressions
- Reader supporting Clojure's rich data literal syntax:
  - Symbols and keywords
  - Integers and floating-point numbers
  - Strings with escape sequences
  - Lists `()`
  - Vectors `[]`
  - Maps `{}`
  - Sets `#{}`

## Getting Started

### Prerequisites

- Rust 1.70 or later

### Installation

```sh
git clone https://github.com/conao3/rust-ruje.git
cd rust-ruje
cargo build --release
```

### Running the REPL

```sh
cargo run
```

Example session:

```clojure
(+ 1 2 3)
[1 2 3]
{:name "ruje" :version "0.1.0"}
#{a b c}
```

## Project Structure

```
src/
  main.rs    - Entry point and REPL loop
  lib.rs     - Library exports
  core.rs    - Read-eval-print pipeline
  reader.rs  - Clojure reader implementation
  types/     - Data type definitions
    atom.rs  - Atomic types (symbols, numbers, strings)
    exp.rs   - Expression types (lists, vectors, maps, sets)
```

## Development

Run tests:

```sh
cargo test
```

Build in release mode:

```sh
cargo build --release
```

## License

Apache-2.0
