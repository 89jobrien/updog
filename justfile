default: ci

# Run all CI checks via cargo xtask (includes rail unify --check)
ci:
    cargo xtask ci

# Run only clippy
lint:
    cargo xtask clippy

# Run tests only
test:
    cargo xtask test

# Run rail CI surface (build + test, all crates, change-aware)
rail-ci:
    cargo xtask rail-ci

# Publish to crates.io via cargo rail release
rail-release:
    cargo xtask rail-release

# Check workspace dependency unification (non-destructive)
rail-check:
    cargo rail unify --check

# Generate SARIF reports from clippy and cargo rail → target/*.sarif
sarif:
    cargo xtask sarif

# Install clippy-sarif if missing, then generate SARIF reports
sarif-check:
    @which clippy-sarif > /dev/null 2>&1 || cargo install clippy-sarif --locked
    cargo xtask sarif

# Run all CI checks via crux pipeline
ci-crux:
    crux run --target ci

# Run with smolvm — isolated Linux VM (requires smolvm on PATH)
ci-vm:
    smolvm machine run --net --image rust:latest -- sh -c \
        "cd /workspace && cargo fmt --all -- --check && \
         cargo clippy --workspace --all-targets -- -D warnings && \
         cargo build --workspace --all-targets && \
         cargo install cargo-nextest --locked && \
         cargo nextest run --workspace"
