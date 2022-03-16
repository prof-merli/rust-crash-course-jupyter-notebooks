# Rust Crash Course Jupyter Notebooks

This is a series of [Jupyter](https://jupyter.org/) notebooks serving as a
crash course, i.e. a compact and straight-forward introduction, for the
[Rust programming language](https://www.rust-lang.org/).

This might be especially useful for interactive Rust introduction sessions,
e.g. in university education or software development teams.

## Requirements

The provided notebooks require these components for correct operation:

 * Rust
 * Jupyter Notebook Server
 * EvCxR Jupyter Kernel
 * Visual Studio Code and its ``rust-analyzer`` Extension
 * ``cargo audit``

### Rust

Rust can be installed by the [``rustup`` tool](https://www.rust-lang.org/learn/get-started)
provided on the official [Rust website](https://www.rust-lang.org/).

### Jupyter Notebook Server

There are several ways to install a Jupyter notebook environment on your system.

Some are shown on the official [Jupyter project website](https://jupyter.org/).

Also, common Linux distributions like Ubuntu provide dedicated packages, e.g.:
```sh
sudo apt install jupyter-notebook
```

### EvCxR Jupyter Kernel

Follow the installation instructions provided by the developers of the
[EvCxR Jupyter kernel](https://github.com/google/evcxr/tree/master/evcxr_jupyter).

### Visual Studio Code and its ``rust-analyzer`` Extension

Visual Studio Code is a popular code editor for Rust. It can be installed on
a variety of platforms as explained in the [VS Code setup documentation](https://code.visualstudio.com/docs/setup/setup-overview).

After starting VS Code, make sure to install the ``rust-analyzer`` extension,
which comes with many supporting features for Rust development.

Further, you might want to check, if these two options are set correctly:
* File > Preferences > Settings > Text Editor > Formatting > Format On Save: **checked**
* File > Preferences > Settings > Extensions > Rust Analyser > Check On Save: Command: **clippy**

### ``cargo audit``

``cargo audit`` is a subcommand of Cargo, but it has to be installed explicitly.
The different installation procedures are explained in the
[``cargo audit`` repository](https://github.com/RustSec/rustsec/tree/main/cargo-audit).

A common way to install ``cargo audit`` is to use ``cargo``:
```sh
cargo install cargo-audit
```

## License

The notebooks are licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

