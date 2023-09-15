#!/usr/bin/env bash
cargo build
target/debug/rust-vm -cp java com.max.Hello
