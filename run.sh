#!/bin/sh

path=$1"/rust_sol"

if [ ! -d $path ]; then
    echo "$path does not exist."
    echo "Please enter file with rust solution."
else
    cd $path
    cargo run
    cd ../..
fi
    
