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

assert "42" "42"
assert "123" "123"
assert "  42 " "42"
assert "12+23" "35"
assert "12+23+34" "69"
assert "23-12" "11"
assert "23-12-2+34" "43"
assert "2*13" "26"
assert "3+2*13-9" "20"
assert "3+2*3*4-7" "20"
assert "6/2" "3"
assert "4/3" "1"
assert "2*3-6/2+1" "4"
assert "1+2*3" "7"
assert "(1+2)*3" "9"

echo OK
