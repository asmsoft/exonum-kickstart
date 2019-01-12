#!/usr/bin/env bash

APP_NAME={{project_name}}
cargo build
rm -rf /tmp/config/$APP_NAME/db/3
#export RUST_LOG="debug"
./target/debug/$APP_NAME run -d /tmp/config/$APP_NAME/db/3/ -c /tmp/config/$APP_NAME/finalize_config.toml --public-api-address 0.0.0.0:8000 --private-api-address 0.0.0.0:8001