# Argot CLI

**Argot CLI** is a lightweight command-line argument parser for Rust, bundled
with a command-line utility for working with Argot configurations.

The project provides:

* a **Rust library** for building CLI parsers
* a **CLI executable** named `argot`
* a small macro-based configuration DSL
* serde support, with JSON and TOML configuration formats

---

## Installation

### Install the CLI

```sh
cargo install argot-cli
```

This installs the executable:

```sh
argot
```

---

### Use as a Rust library

```sh
cargo add argot-cli
```

Or add directly to your `Cargo.toml`:

```toml
[dependencies]
argot-cli = "0.1"
```

---

## Quick Example (Library)

```rust
use argot_cli::config;

let config = config! {
    "quiet"       => Flag,
    "count"       => Count,
    "port"        => Int { default: 8080 },
    "config-file" => Text { default: "config.toml" },
    "include"     => List { sep: "," },
    "q"           => Alias { target: "quiet" },
};
```

This produces a `ParserConfig` used by the parser.

---

## Status

Early development. APIs may change.
