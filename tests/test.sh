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

cargo b

assert "42" "42"
assert "123" "123"

echo OK
