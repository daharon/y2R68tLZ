# Parallel Hashing

Download URLs in parallel and print the MD5 hashes of their response bodies to `STDOUT`.   
Debug messages are printed to `STDERR`.

## Usage
```text
phash 0.1.0
Dan Aharon <dan@aharon.dev>
Parallel Hashing

USAGE:
    phash [OPTIONS] <FILE>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <NUM>     [default: 1]

ARGS:
    <FILE>...    Text file(s) containing URLs separated by newlines
```

## Run
```shell
cargo run -- --concurrency=10 ./test.txt
```