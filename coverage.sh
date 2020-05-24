#!/usr/bin/env bash
# referenced by https://blog.esplo.net/entry/2019/04/grcov-script

set -eux

PROJ_NAME=$(cat Cargo.toml | grep -E "^name" | sed -E 's/name[[:space:]]=[[:space:]]"(.*)"/\1/g' | sed -E 's/-/_/g')
rm -rf target/debug/deps/${PROJ_NAME}-*

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off"

cargo +nightly build --verbose
cargo +nightly test --verbose

grcov ./target/debug/deps -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --excl-start GRCOV_EXCL_START --excl-stop GRCOV_EXCL_STOP -o lcov.info
genhtml -o report/ --show-details --highlight --ignore-errors source --legend lcov.info
