# Lottery
Does quick picks for Megamillions and Powerball if you don't trust the machines at the quick stop.

```
user@host$ cargo run -q
Winning numbers:
8-19-24-54-64 (16)
```

```
user@host$ cargo run -q -- --help
lottery 0.0.3

USAGE:
    lottery [FLAGS]

FLAGS:
    -h, --help            Prints help information
    -m, --megamillions    Megamillions Picks
    -p, --powerball       Powerball Picks
    -V, --version         Prints version information
```

## Building, testing, and running
[Make sure you have rust installed](https://www.rust-lang.org/tools/install)

## Build
```
cargo build
```

## Test
```
cargo test
```

## Run
```
cargo run [-- <lottery flags>]
```

## Install
```
cargo build --release && sudo cp target/release/lottery /usr/local/bin/lottery
```
