#!/bin/bash

echo "| Day (a) | Part 1 (b=0) | Part 2 (b=1) |"
echo "|---------|--------------|--------------|"

extract_mean_time() {
  local result=$(hyperfine -m 1000 -N --warmup 10 "$1" --time-unit microsecond 2>/dev/null | grep "Time (mean ± σ)")
  local mean_time=$(echo "$result" | awk '{print $5}')
  echo "$mean_time µs"
}

for a in {1..24}; do
  command_b0="target/release/aoc2024 $a 0 0 0"
  command_b1="target/release/aoc2024 $a 1 0 0"

  part1_time=$(extract_mean_time "$command_b0")
  part2_time=$(extract_mean_time "$command_b1")

  echo "| $a       | $part1_time     | $part2_time     |"
done