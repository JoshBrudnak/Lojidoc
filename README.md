# Lojidoc

A tool for generating markown documentation for java projects.

[![Build Status](https://travis-ci.org/JoshBrudnak/Javadoc-To-Markdown.svg?branch=master)](https://travis-ci.org/JoshBrudnak/Javadoc-To-Markdown)

## Documentation

[Lojidoc](https://joshbrudnak.github.io/lojidoc/)

## Contribution

Pull requests and feature requests are always welcome.

## Building

```
$ cargo build --release
```

## Usage

Basic Usage

```
$ ./lojidoc [Project_Path]
```

| Flag | Description                                             |
| ---- | ------------------------------------------------------- |
| s    | Use only on thread for execution of the program         |
| l    | Check a java project for incorrent and missing javadocs |
| h    | Prints help information                                 |

| Options | Description                                                    |
| ------- | -------------------------------------------------------------- |
| c       | Add the git repository url of the project to the documentation |
| d       | Sets the destination directory of the created markdown files   |
