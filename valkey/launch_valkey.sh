#!/bin/bash

clear

MOD="/home/matteo/valkey-json/build/src/libjson.so"

[ ! -f "$MOD" ] && echo "Module not found!" && exit 1

echo "Starting Valkey..."
valkey-server --loadmodule "$MOD" --daemonize yes
until valkey-cli ping &>/dev/null; do sleep 0.2; done
valkey-cli