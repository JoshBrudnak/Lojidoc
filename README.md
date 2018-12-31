# Lojidoc

[![lojidoc on crates.io](http://meritbadge.herokuapp.com/lojidoc)](https://crates.io/crates/lojidoc) [![Build Status](https://travis-ci.org/JoshBrudnak/Lojidoc.svg?branch=master)](https://travis-ci.org/JoshBrudnak/Lojidoc)

## About

Lojidoc is a tool for creating markdown documentation for java projects. The
program parses the javadoc comments in the source code and uses the
high level java declarations to provide consice and easily readable markdown
documentation for each java file in the project.

## Documentation

[Source code documentation](https://joshbrudnak.github.io/Lojidoc/)

## Contribution

Pull requests and feature requests are always welcome. If you are opening an issue please use one of the provided issue templates as a guide. For more information about contributing read the [Contribution guidelines](https://github.com/JoshBrudnak/Lojidoc/blob/master/CONTRIBUTING.md).

## Installation

Lojidoc can be installed using [Cargo](http://crates.io/lojidoc). The following packages are required for installation.

- rustc [[installation]](https://www.rust-lang.org/en-US/install.html)
- cargo [[installation]](https://doc.rust-lang.org/cargo/getting-started/installation.html)

> Note: cargo is installed by default with a rustc installation

Lojidoc can then be installed using `cargo install`
```bash
$ cargo install lojidoc
```
Once installed the default location of the executable will be `~/.cargo/bin/lojidoc`

## Building from source

```bash
$ git clone https://github.com/JoshBrudnak/Lojidoc.git
$ cd ./Lojidoc/
$ cargo build --release
```

## Usage

#### Basic Usage

```bash
$ lojidoc [Project_Path] [FLAGS] [OPTIONS]
```

#### Example Usages

Example of passing Lojidoc the repository URL and destination directory
```bash
$ lojidoc ~/Project/src/java/ -c https://github.com/JoshBrudnak/Project/tree/master -d ~/docs/
```


Example of using the lint option to find javadoc mistakes and using a single thread.
```bash
$ lojidoc ~/Project/src/java/ -l -s
```
> Note: Lojidoc will not generate any markdown files when using the lint flag

## Command line arguments

| Flag | Description                                                      |
| ---- | ---------------------------------------------------------------- |
| c    | Delete the destination directory before generating documentation |
| m    | Use multiple threads to execute the program                      |
| l    | Check a java project for incorrect or missing javadocs           |
| h    | Prints help information                                          |
| v    | Generate documentation for a project and provide verbose output  |
| V    | Prints the version information                                   |

| Option     | Description                                                    |
| ---------- | -------------------------------------------------------------- |
| d <FILE>   | Sets the destination directory of the created markdown files   |
| b <FILE>   | Create a mdbook using the generated documentation              |
| i <STRING> | Ignore fields with a certain permission
