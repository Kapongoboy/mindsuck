# Mindsuck

A brainf**ck interpreter inspired by [Krzysztof Gabis's](http://github.com/kgabis/brainfuck-c) interpreter in c, except that mine kind of sucks.

Written in pure rust, it can run hello world but give it anything with comments
or other illegal characters and it falls apart ;-;

## Usage

Compile it easily with cargo:

```bash
cargo build --release
```

Then run it with the path to the brainf**ck file as the first argument:

```bash
./target/release/mindsuck examples/hello.bf
```

You can also run it with cargo run like so:

```bash
cargo run --release -- examples/hello.bf
```
