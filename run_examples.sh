#!/bin/bash

bin_dir="target/debug/examples"
src_dir="examples"

if ! [ -d "$src_dir" ] ; then
    echo "Directory \"$src_dir\" does not exist."
    exit 1
fi
cargo build --examples
if ! [ -d "$bin_dir" ] ; then
    echo "Directory \"$bin_dir\" does not exist."
    echo "Error"
    exit 1
fi

readarray -t bins < <( find examples -type f -name "*.rs" )

for idx in "${!bins[@]}"; do
    bin=$( basename -s ".rs" "${bins[idx]}")
    if ! [ -f "$bin_dir/$bin" ] ; then
        echo "File $bin_dir/$bin does not exist."
        echo "Error"
        exit 1
    fi
    echo "################################"
    echo "### TEST FOR: $bin"
    echo "################################"
    "./$bin_dir/$bin"
    echo -e "\\n\\n"
done
