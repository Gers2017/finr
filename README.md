# Finr

Recursively search for files and directories with a pattern while ignoring irrelevant directories.

## Table of contents

-   [Why](#why)
-   [What about find](#is-this-another-find-replacement)
-   [Installation](#installation)
-   [Build from source](#build-from-source)
-   [Usage](#usage)
-   [Similar tools](#similar-tools)

## Comparing [finr](https://crates.io/crates/finr) and and [fd-find](https://crates.io/crates/fd-find)

![finr benchmark](./assets/hyperfine_all.png)

## Why

I made finr to suit my own needs, and because I was tired of getting excessive results from find.

-   Because I wanted a fast tool that suited my needs.
-   Because if I keep using someone else's tool how am I supposed to learn how to build one myself.

### Is this another find replacement?

Nope. I consider `find` to be a great tool with more features than `finr`.
Since finr is relatively new it doesn't support as many features as find or fd-find (so keep that in mind).

## Installation

Assumes that you have rust and cargo installed.

```sh
cargo install finr
```

## Build from source

```sh
git clone https://github.com/Gers2017/finr && \
cd finr && \
cargo build --release
```

## Usage

Print help message

```sh
finr --help
```

By default finr searches for **files** and starts at the **current directory**.
If you want to search for a directory use `-t d` (--type directory).
The max-depth is arbitrarily set to 100.

Search for .rs files using regex (Uses the [regex crate](https://crates.io/crates/regex))

```sh
finr '.+\.rs$' --regex
```

Search for files with `.rs`. Starting at the current directory. (Uses [ends_with](https://doc.rust-lang.org/std/string/struct.String.html#method.ends_with))

```sh
finr .rs -e
```

Search for directories that contain `_node_modules_` in the name.

```sh
finr node_modules -t d
```

Searching for files that contain `main` in the name (Uses [contains](https://doc.rust-lang.org/std/string/struct.String.html#method.contains))

```sh
finr main
```

Search for files with `.rs` starting at the /home/ directory while ignoring some directories.

```sh
finr .rs ~/ -e -i Files Videos Downloads .config .local
```

Search for files that contain `main.c` starting at the current directory. Ignoring `Music Videos Downloads` and Including `.config .local .ignore`.

```sh
finr main.c --ignore Music Videos Downloads --include .config .local .ignore
```

## Similar tools

-   [fd-find](https://crates.io/crates/fd-find)
-   [ff](https://github.com/vishaltelangre/ff)
-   [rsfind](https://github.com/willshuttleworth/rsfind)
