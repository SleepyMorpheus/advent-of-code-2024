#!/bin/bash

# Iterate over a from 1 to 24
for a in {1..24}; do
  # Iterate over b from 0 to 1
  for b in {0..1}; do
    hyperfine -m 1000 -N --warmup 100 "target/release/aoc2024 $a $b 0 0" 2> /dev/null
  done
done