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

assert "main() { return 42; }" "42"
assert "main() { return 123; }" "123"
assert "main() {   return 42;  }" "42"
assert "main() { return 12+23; }" "35"
assert "main() { return 12+23+34; }" "69"
assert "main() { return 23-12; }" "11"
assert "main() { return 23-12-2+34; }" "43"
assert "main() { return 2*13; }" "26"
assert "main() { return 3+2*13-9; }" "20"
assert "main() { return 3+2*3*4-7; }" "20"
assert "main() { return 6/2; }" "3"
assert "main() { return 4/3; }" "1"
assert "main() { return 2*3-6/2+1; }" "4"
assert "main() { return 1+2*3; }" "7"
assert "main() { return (1+2)*3; }" "9"
assert "main() { 1; 2; return 3; }" "3"
assert "main() { ; return 3; }" "3"
assert "ret() { return 42; } main() { return 123; }" "123"
assert "ret() { return 42; } main() { return ret(); }" "42"
assert "main() { 1; return 2; 3; }" "2"
assert "main() { 1; return 2; return 3; }" "2"

echo OK
