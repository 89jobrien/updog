default: ci

# Run all CI checks via cargo xtask
ci:
    cargo xtask ci

# Run only fmt + clippy
lint:
    cargo xtask clippy

# Run tests only
test:
    cargo xtask test

# Run all CI checks via crux pipeline
ci-crux:
    crux run --target ci

# Run with smolvm — isolated Linux VM (requires smolvm on PATH)
ci-vm:
    smolvm machine run --net --image rust:latest -- sh -c \
        "cd /workspace && cargo fmt --all -- --check && \
         cargo clippy --workspace --all-targets -- -D warnings && \
         cargo build --workspace --all-targets && \
         cargo test --workspace"
