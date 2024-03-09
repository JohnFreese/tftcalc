run:
    cargo run 

watch:
    cargo watch -x run

run-release:
    cargo run --release

build:
    cargo build

build-release:
    cargo build --release

test: 
    cargo test

watch-test:
    cargo watch -x test

setup-hooks:
    rm -rf .git/hooks && ln -sr ./hooks .git/hooks
