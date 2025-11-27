#!/usr/bin/env zsh

test_compiler="./writing-a-c-compiler-tests/test_compiler"

compiler="$(cargo metadata --format-version 1 | jq -r '.target_directory')/debug/seacat"

"$test_compiler" "$compiler" --chapter 1 --stage lex
"$test_compiler" "$compiler" --chapter 1 --stage parse
"$test_compiler" "$compiler" --chapter 1 --stage codegen



