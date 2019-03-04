#!/bin/sh

set -ex

cargo test --target $TARGET --no-run --features subscriber

if [ -z "$NO_RUN" ]; then
    cargo test --target $TARGET --features subscriber
fi
