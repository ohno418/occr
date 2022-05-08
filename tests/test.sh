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

assert "int main() { return 42; }" "42"
assert "int main() { return 123; }" "123"
assert "int main() { return 12+23; }" "35"
assert "int main() { return 12  +23+ 34; }" "69"
assert "int main() { return 23-12; }" "11"
assert "int main() { return 23-12-2+34; }" "43"
assert "int main() { return 2*13; }" "26"
assert "int main() { return 3+2*13-9; }" "20"
assert "int main() { return 3+2*3*4-7; }" "20"
assert "int main() { return 6/2; }" "3"
assert "int main() { return 4/3; }" "1"
assert "int main() { return 2*3-6/2+1; }" "4"
assert "int main() { return 1+2*3; }" "7"
assert "int main() { return (1+2)*3; }" "9"
assert "int main() { 1; 2; return 3; }" "3"
assert "int main() { ; return 3; }" "3"
assert "int ret() { return 42; } int main() { return 123; }" "123"
assert "int ret() { return 42; } int main() { return ret(); }" "42"
assert "int main() { 1; return 2; 3; }" "2"
assert "int main() { 1; return 2; return 3; }" "2"
assert "int main() { 1; { 2; 3; return 4; } return 5; }" "4"

echo OK
