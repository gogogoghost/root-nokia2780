#!/bin/sh

# copy to device

set -e

ROOT=/run/media/ghost/_

sudo cp libs/armeabi-v7a/su $(ROOT)/system/xbin/su
sudo cp libs/armeabi-v7a/su-daemon $(ROOT)/system/xbin/su-daemon