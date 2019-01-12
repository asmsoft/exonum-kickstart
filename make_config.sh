#!/usr/bin/env bash

APP_NAME={{project_name}}
./target/debug/$APP_NAME generate-template /tmp/config/$APP_NAME/config.toml --validators-count 1
./target/debug/$APP_NAME generate-config /tmp/config/$APP_NAME/config.toml /tmp/config/$APP_NAME/pub_config.toml /tmp/config/$APP_NAME/sec_config.toml --peer-address 127.0.0.1:6334
./target/debug/$APP_NAME finalize /tmp/config/$APP_NAME/sec_config.toml /tmp/config/$APP_NAME/finalize_config.toml -p /tmp/config/$APP_NAME/pub_config.toml