# Mindsuck

A brainf**ck interpreter inspired by [Krzysztof Gabis's](http://github.com/kgabis/brainfuck-c) interpreter in c, except that mine ~~kind of~~ sucks (not anymore it doesn't 😎).

Written in pure rust, It can run bottles.bf, which is pretty cool

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
cargo run --release -- examples/bottles.bf
```

For my python enjoyers I've included as an extra treat

```bash
python src/whynot.py -- examples/bottles.bf
```

And that's all she wrote
