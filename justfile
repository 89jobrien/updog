default: ci

# Run all CI checks locally via crux
ci:
    crux run --target ci

# Run only lint checks
lint:
    crux run --target lint

# Run tests only
test:
    crux run --target test

# Run with smolvm — isolated Linux VM (requires smolvm on PATH)
ci-vm:
    smolvm machine run --net --image rust:latest -- sh -c \
        "cd /workspace && cargo fmt --all -- --check && \
         cargo clippy --workspace --all-targets -- -D warnings && \
         cargo build --workspace --all-targets && \
         cargo test --workspace"
