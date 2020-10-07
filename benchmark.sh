#!/bin/bash

cd $(dirname `realpath $0`)

lastgid=`git rev-parse --short HEAD`

if [[ $(ls -R target/criterion/ | grep $lastgid) ]]; then
    echo "baseline $lastgid"
    cargo bench -- --baseline "$lastgid"
else
    echo "creating baseline $lastgid"
    cargo bench -- --save-baseline "$lastgid"
fi
