#!/bin/bash
#IP=0.0.0.0 PORT=8080 cargo run --release --features server
dx bundle --platform web
cd target/dx/steampick/release/web/
IP=0.0.0.0 PORT=8080 ./server
