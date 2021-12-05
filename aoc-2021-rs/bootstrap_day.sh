#!/usr/bin/env bash

set -euo pipefail

export DAY=${1:-undefined}

if [ "${DAY}" = "undefined" ]; then
    echo "Usage: $0 <dayX>"
fi

export DAY_NUM=${DAY#"day"}

# Create source file from template
envsubst < src/dayX.rs.tmpl > src/${DAY}.rs

# Create new binary target
envsubst <<EOF >> Cargo.toml

[[bin]]
name = "${DAY}"
path = "src/${DAY}.rs"
EOF

# Open the puzzle of the day
xdg-open https://adventofcode.com/2021/day/${DAY_NUM}
