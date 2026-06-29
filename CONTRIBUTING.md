# Contributing

## Prerequisites

- Rust stable (for builds) and nightly (for clippy, bench, udeps)
- [`just`](https://github.com/casey/just) — command runner
- Tools installed via `cargo install`: `cargo-audit`, `cargo-deny`, `cargo-udeps`, `cargo-geiger`, `cargo-llvm-cov`

## Development

```sh
just lint   # fmt --check + clippy + doc
just test   # doc-tests + all-targets + bench
just sec    # audit + deny + udeps + geiger
just cov    # llvm-cov HTML report
just open   # open the coverage report
just all    # lint + test + sec + cov
```

Before committing, run `just all` and ensure all checks pass.

## Guidelines

- This crate is `#![no_std]` and `#![forbid(unsafe_code)]` — no std-only APIs, no raw pointers.
- All parsing is zero-copy; prefer references and array copies over allocation.
- Keep `cargo deny check` clean — no new dependencies without review.
- Match existing code style; run `cargo fmt --all` before committing.
