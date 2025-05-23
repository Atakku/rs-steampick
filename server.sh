#!/bin/bash
dx bundle --platform server
cd target/dx/steampick/release/web/
IP=0.0.0.0 PORT=8080 ./server
