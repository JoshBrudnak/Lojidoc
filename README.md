# Lojidoc

[![lojidoc on crates.io](http://meritbadge.herokuapp.com/lojidoc)](https://crates.io/crates/lojidoc) [![Build Status](https://travis-ci.org/JoshBrudnak/Lojidoc.svg?branch=master)](https://travis-ci.org/JoshBrudnak/Lojidoc)

## About

Lojidoc is an tool for creating markdown documentation for java projects. The program reads the javadoc comments in the source code and uses some of the high level java declarations to provide consice and easily readable markdown documentation for each java file in the project. Lojidoc also allows the user to lint their javadocs and find all the areas project that are undocumented or have incorrect documentation.

## Documentation

[Lojidoc](https://joshbrudnak.github.io/Lojidoc/)

## Contribution

Pull requests and feature requests are always welcome. Please read the [Contribution guidelines](https://github.com/JoshBrudnak/Lojidoc/blob/master/CONTRIBUTING.md)

## Installation

This application can be installed with [Cargo](http://crates.io).

```toml
lojidoc = "0.1.0"
```

## Building from source

```bash
$ git clone https://github.com/JoshBrudnak/Lojidoc.git
& cd ./Lojidoc/
$ cargo build --release
```

## Usage

Basic Usage

```
$ ./lojidoc [Project_Path]
```

Other options

| Flag | Description                                             |
| ---- | ------------------------------------------------------- |
| s    | Use only on thread for execution of the program         |
| l    | Check a java project for incorrent and missing javadocs |
| h    | Prints help information                                 |

| Options | Description                                                    |
| ------- | -------------------------------------------------------------- |
| c       | Add the git repository url of the project to the documentation |
| d       | Sets the destination directory of the created markdown files   |
