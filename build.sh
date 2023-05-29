#!/usr/bin/env bash

set -Eeuo pipefail

function check_installed {
    tools=( "cargo" "cargo fmt" "cargo build" "cargo clippy" "cargo check" "cargo audit" "cargo deny" )
    for index in "${!tools[@]}" ; do
        tool=${tools[$index]}
        $tool --help > /dev/null || { echo "Please ensure that $tool is installed!"; exit -1; }
    done
}

# we check that all required cargo extensions are already installed
check_installed

# we check, fix, format, and build the code
cargo update
# try to fix first 
cargo clippy --fix --allow-dirty -- -D warnings
# fail on any warning that is left
cargo clippy -- -D warnings
cargo fmt
cargo build --future-incompat-report
cargo +nightly udeps
cargo clippy  -- -D clippy::pedantic
cargo check --all
cargo audit
cargo deny check --config deny.toml
cargo test
cargo doc
