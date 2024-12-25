<h1><code>.bf</code></h1>

Brainfuck interpreter written in Rust for fun over the evening.

The ```examples/``` directory has some example ```.bf``` programs for you to try out and have fun. They all come from [this](https://en.wikipedia.org/wiki/Brainfuck) Wikipedia page.

> [!NOTE]
> I didn't put much effort into making sure that this is super error proof so stuff might explode if you push it too much. Have fun!

## Requirements

→ [Rust](https://www.rust-lang.org/)\
→ [Cargo](https://doc.rust-lang.org/cargo/)

## Download

Download the source code using the ```git clone``` command:

```bash
$ git clone https://github.com/wedkarz02/dotbf.git
```

Or use the *Download ZIP* option from the Github repository [page](https://github.com/wedkarz02/dotbf.git).

## Building

Build the application using ```cargo``` in release mode:

```bash
$ cargo build --release
```

Release mode is required due to Rust's integer overflow quirks.

The binary is self-contained so you can easily copy / move / symlink it from the ```target/``` directory:

```bash
$ cp ./target/release/dotbf ~/.local/bin
$ mv ./target/release/dotbf ~/.local/bin
$ ln ./target/release/dotbf ~/.local/bin
```

## Usage

There's really not much to it:

```bash
# Usage: dotbf <FILE PATH>
$ dotbf main.bf
```

## License

If not directly stated otherwise, everything in this project is under the MIT License. See the [LICENSE](https://github.com/wedkarz02/dotbf/blob/main/LICENSE) file for more info.
