#!/usr/bin/env bash

# Prints the Solana version.

set -e

cd "$(dirname "$0")/../../plugin"

cargo read-manifest | jq -r '.dependencies[] | select(.name == "agave-geyser-plugin-interface") | .req'