# RPN Calculator In Rust!

> This is a program that parses a RPN Notation Equation and prints
> the result. Based off of [rpn.py](https://gist.github.com/wd5gnr/68d067c3c42a2e0e9a27b083e01f7080#file-rpn-py) by [@wd5gnr](https://github.com/wd5gnr).

## Heads Up!

This is a POC! Features may be added or removed as seen fit.

## LICENSE

See the [LICENSE](./LICENSE) file. TLDR - MIT License

## Usage

```
Usage: rpn_calculator.exe [OPTIONS]

Options:
  -e, --expression <EXPRESSION>  Reverse Polish Notation Equation
  -t, --test-info                Show some test info and exit.
  -h, --help                     Print help
  -V, --version                  Print version
```

# Building

Use `cargo build` to build the source.

```shell
cd <Checkout directory>
cargo run -- --expression "2 8 +"
```

## Valid Operators

As of 02/23/2024, [rpn-rs](./src/lib.rs) understands the following operators:

| Operator | Description                               |
| :------: | :---------------------------------------- |
|    +     | Addition                                  |
|    -     | Subtraction                               |
|    \*    | Multiplication                            |
|    /     | Division                                  |
|    ^     | Power Of                                  |
|    !     | Store Top Of Stack in a tempVar           |
|    @     | Retrieve tempVar and push to Top Of Stack |
|    ?     | Dump the stack                            |
|    &     | Dump the tempVar list                     |
