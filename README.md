# build-cxx

A Base of a Cargo crate with cxx already configured.

## Table of Contents

- [Introduction](#introduction)
- [Getting Started](#getting-started)

## Introduction

The build-cxx repository is a base template for creating Rust libraries that can be integrated with C++ projects. This setup includes necessary configurations for Rust and C++ interoperation using the `cxx` crate.

## Getting Started

### Prerequisites

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- C++ compiler: Ensure you have a compatible C++ compiler installed

### Cloning the Repository

To clone the repository with the `bin` tag, use the following command:
```sh
git clone --branch bin --depth 1 https://github.com/MoutOgm/build-cxx.git
cd build-cxx
```

To clone the repository with the `lib` tag, use the following command:
```sh
git clone --branch lib --depth 1 https://github.com/MoutOgm/build-cxx.git
cd build-cxx
```

If you are cloning this repository into an existing Git repository, remove the .git directory:
```sh
rm -rf build-cxx/.git
```

### Dependencies

Ensure you have the following dependencies specified in your `Cargo.toml` file:
```toml
[dependencies]
cxx = "1.0.129"

[build-dependencies]
cxx-build = "1.0.129"
```

### Building the Project

To build the project, use Cargo:
```sh
cargo build --release
```

### Running the Project

To run the project, use:
```sh
cargo run
```

### Integrating with C++

Use `cxx` to integrate Rust with your C++ codebase. Refer to the `cxx` [documentation](https://cxx.rs) for more details on how to set this up.
