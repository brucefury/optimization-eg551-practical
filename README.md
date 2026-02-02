# EG551 Practical — Optimization Algorithms

## Build & Run

```bash
# Run a specific week (with plots)
cargo run --features plotting -- <week>

# Examples
cargo run --features plotting -- 1   # Week 1: Graphing
cargo run --features plotting -- 2   # Week 2: Neville's Interpolation
```

Output goes to `output/week<N>/`.

## Tests

```bash
cargo test
```
