#!/bin/sh

set -e;

cargo build;

sudo ./target/debug/su --daemon