#!/bin/bash

assert() {
  input="$1"
  expected="$2"

  ./target/debug/rocc "$input" > ./tests/tmp.s
  gcc -o ./tests/tmp ./tests/tmp.s
  ./tests/tmp
  actual="$?"

  if [ "$actual" = "$expected" ]
  then
    echo "$input => $actual"
  else
    echo "$input => expected $expected, but got $actual"
    exit 1
  fi
}

cargo build

assert "main() { 42; }" "42"
assert "main() { 123; }" "123"
assert "main() {   42;  }" "42"
assert "main() { 12+23; }" "35"
assert "main() { 12+23+34; }" "69"
assert "main() { 23-12; }" "11"
assert "main() { 23-12-2+34; }" "43"
assert "main() { 2*13; }" "26"
assert "main() { 3+2*13-9; }" "20"
assert "main() { 3+2*3*4-7; }" "20"
assert "main() { 6/2; }" "3"
assert "main() { 4/3; }" "1"
assert "main() { 2*3-6/2+1; }" "4"
assert "main() { 1+2*3; }" "7"
assert "main() { (1+2)*3; }" "9"
assert "main() { 1; 2; 3; }" "3"
assert "main() { ; 3; }" "3"
assert "ret() { 42; } main() { 123; }" "123"
assert "ret() { 42; } main() { ret(); }" "42"

echo OK
