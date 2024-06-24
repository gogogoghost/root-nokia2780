#!/bin/sh

# build for local test

set -e

mkdir -p build/
gcc jni/su-daemon.c -o build/su-daemon
gcc jni/su.c -o build/su