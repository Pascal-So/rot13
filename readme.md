# Rot13

A simple rot13 tool for practicing rust. Applies rot13 to characters in the ranges [a..z] and [A..Z], leaving all the other characters untouched.

## Build

```bash
cargo build --release
```

## Usage

Reads from stdin and prints to stdout.

```bash
$ echo "Hello World! И это не меняется" | ./rot13
Uryyb Jbeyq! И это не меняется
```