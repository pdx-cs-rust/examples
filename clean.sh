#!/bin/sh
for i in *.rs
do
    rm -f `basename $i .rs`
done
for i in */Cargo.toml
do
    ( cd `dirname $i` && cargo clean -q )
done
