# Rust Load Balancer

[] YAML Parser
[] LB INITIALIZE
[] LB ALGORITHMS [ROUND ROBIN, LEAKY BUCKET ETC]
[] MORE!!!
[] YAML OR CMD LINE ARGUMENTS (EXTENSION)

### Day 1

```
   lb-core -> will contain lb algorithms etc
   lb-app -> starting point of the app
   yml-parser -> yaml parser ofc
```

- created a rust workspace
  - create Cargo.toml file, include members (lb-core, lb-app, yml-parser)
  - cargo new --lib lb-core to create lib (lb-core, yml-parser)
  - cargo new --bin lb-app to create starting point of the app (main)

### Day 2

- created preprocess_by_line() and added tests in parser.rs

### Instructions

- To build the app and run, `cargo build`, `cargo run`
