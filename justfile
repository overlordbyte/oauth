# just dev — backend + frontend bir vaqtda ishga tushadi
# O'rnatish: cargo install just

dev:
    cargo run & cd frontend && dx serve

build:
    cargo build --release
    cd frontend && dx build --release

check:
    cargo check --workspace
