#!/bin/bash

lastgid=`git rev-parse HEAD`
cargo bench #-- --baseline "$lastgid"
