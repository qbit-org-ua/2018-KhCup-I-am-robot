#!/bin/sh

set -x -e -o pipefail

if [ -f "$1" ]; then
    SOLUTION=$1
else
    rustc ./solutions/author.rs
    SOLUTION=./author
fi

rm -f /tmp/lines-generator-state.txt
> /tmp/board.log

export MOVES_PER_GAME=10000
for i in `seq 1000` ; do
    ./lines-generator/target/debug/lines-generator /tmp/lines-generator-state.txt /tmp/output.txt | tee /tmp/input.txt | tee -a /tmp/board.log
    "$SOLUTION" < /tmp/input.txt | tee /tmp/output.txt | tee -a /tmp/board.log
done

echo "NOTE: The game log is available at /tmp/board.log"

./lines-checker/target/debug/lines-checker /tmp/input.txt /tmp/output.txt ../100.out
