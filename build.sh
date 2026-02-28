#!/bin/bash
cargo build --profile release-max --bin steampick_client --target x86_64-unknown-linux-gnu --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release-max/steampick_client target/steampick_client-amd64linux
cp target/x86_64-pc-windows-gnu/release-max/steampick_client.exe target/steampick_client-amd64win.exe
