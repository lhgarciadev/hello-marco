[![Rustfmt](https://github.com/lhgarciadev/hello-marco/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/lhgarciadev/hello-marco/actions/workflows/rustfmt.yml)
[![Clippy](https://github.com/lhgarciadev/hello-marco/actions/workflows/lint.yml/badge.svg)](https://github.com/lhgarciadev/hello-marco/actions/workflows/lint.yml)
[![Tests](https://github.com/lhgarciadev/hello-marco/actions/workflows/tests.yml/badge.svg)](https://github.com/lhgarciadev/hello-marco/actions/workflows/tests.yml)
[![Build binary release](https://github.com/lhgarciadev/hello-marco/actions/workflows/release.yml/badge.svg)](https://github.com/lhgarciadev/hello-marco/actions/workflows/release.yml)

# Hello Marco

A simple Marco Polo game implemented in Rust.

## Description

This project is a command-line application that plays a simple Marco Polo game. If the name "Marco" is given, the response will be "Polo". If the name is not "Marco", the response will be "What?".

## Installation

To install the project, you need to have Rust and Cargo installed. You can install Rust and Cargo by following the instructions on the [official Rust website](https://www.rust-lang.org/).

Clone the repository:

```sh
git clone https://github.com/yourusername/hello-marco.git
cd hello-marco
```

## Usage

To run the project, use the following command:

```sh
cargo run -- play --name <name>
```

Replace `<name>` with the name you want to use. For example:

```sh
cargo run -- play --name Marco
```

## Development

To format the code, run:

```sh
make format
```

To lint the code, run:

```sh
make lint
```

To run tests, run:

```sh
make test
```

To build the project, run:

```sh
make release
```

## Running in Codespaces

This project is configured to run in GitHub Codespaces. To get started, open this repository in a Codespace. The development container will be built automatically, and you can start working on the project right away.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

## License

This project is licensed under the terms of the [CC0 1.0 Universal (CC0 1.0) Public Domain Dedication](LICENSE).
