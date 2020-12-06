#!/bin/bash

RELEASES="dc0de41 9a5c378"

TARGET="$(pwd)"
LOVM2=$(realpath "$(pwd)"/..)

#unlink $TARGET/lovm2/.git
rm -rf $TARGET/lovm2

rsync -av --progress $LOVM2 $TARGET \
    --exclude bench \
    --exclude .pytest_cache \
    --exclude .github \
    --exclude target

#ln -s $LOVM2/.git $TARGET/lovm2/.git

rm -r $TARGET/target/criterion

for release in $RELEASES; do
    pushd $TARGET/lovm2
    git checkout -f $release
    VERSION=cat Cargo.toml | awk '$1 == "version" {print$3}' | xargs echo
    popd

    cargo bench -- --save-baseline $VERSION
done

x-www-browser $TARGET/target/criterion/report/index.html &
