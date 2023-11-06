#!/bin/bash

set -a
source .env
set +a

for i in $(seq 19 25); do
  echo "Day $i"
  curl "https://adventofcode.com/2022/day/$i/input" \
    -H "Cookie: $COOKIE" \
    -Lo "inputs/day${i}.txt"
done
