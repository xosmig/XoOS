# xo_os

# Building

WARNING: xargo must be installed

You can install it by running `cargo install xargo`

If there are any problems with installation, look at
https://github.com/japaric/xargo#dependencies

# Makefile
## environment variables:
* CFG='opt1 opt2'
   * Available options: gdb, os_test
* RUSTFLAGS

## Useful targets:
* build &mdash; (default) build debug target
* run &mdash; build debug target and run it via qemu
* tests &mdash; run unit tests
