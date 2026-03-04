all: lint test sec cov

run:
    cargo run

lint:
    cargo fmt --all -- --check
    cargo clippy --all-targets --all-features -- -D warnings

test:
    cargo test --all-targets --all-features

cov:
    cargo llvm-cov --all-features --html --output-dir target/coverage
    cargo llvm-cov report

open:
    cargo llvm-cov report --open

sec:
    cargo audit
    cargo deny check
    cargo +nightly udeps --all-targets
    cargo geiger --all-features
