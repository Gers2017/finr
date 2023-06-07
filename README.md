# Finr

Finr is a command line tool that recursively searches files and directories with a given pattern. Built with the phrase "Work smarter not harder" in mind.

## Table of contents

-   [Installation](#installation)
-   [Why](#why)
-   [Usage](#usage)
-   [Similar tools](#similar-tools)

## Comparing [finr](https://crates.io/crates/finr) and and [fd-find](https://crates.io/crates/fd-find)

![finr benchmark](./assets/hyperfine_all.png)

## Why

Because I wanted a tool that was fast and suited my needs.
Because if I keep using someone else's tool how am I supposed to learn how to build one myself.

## Installation

Assumes that you have rust and cargo installed.

```sh
cargo install finr
```

## Usage

Print help message

```sh
finr --help
```

Search for files with `.rs`. Starting at the current directory.
By default finr searches for files and starts at the current directory.
If you want to search for a directory use `-t d` instead.
The max-depth is arbitrarily set to 100.

```sh
finr .rs -e
```

Search for directories that contain `_node_modules_` in the name.

```sh
finr node_modules -t d
```

Searching for files that contain `main` in the name

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

### Is this another find replacement?

Nope. I consider Find to be a great tool with more features than Finr.
Since Finr is new at this point, it is more of an experiment than a serious well-tested tool.
