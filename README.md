# Rust Figlet

![GitHub release (latest by date)](https://img.shields.io/github/v/release/shruthikmusukula/rust_figlet)
![Language](https://img.shields.io/badge/language-rust-orange)
![Platforms](https://img.shields.io/badge/platforms-Windows%2C%20macOS%20and%20Linux-blue)

* Overview
* Dependencies
* Installation
* Usage
* Additional Info

## Overview 
A cross-platform CLI tool written to bring [FIGlet](http://www.figlet.org) to your machine. Constructed using the Rust programming language.

## Dependencies
There are no depedencies required to install this CLI tool on your machine. However, if you would like to develop it further on your local machine, you will need
to use the Rust programming language.
- [Rust Installation Guide](https://www.rust-lang.org/tools/install)
  - Keep in mind, Rust can also be installed using Homebrew using ```brew install rust```
 
Some additional dependencies to be aware of in this project are the two following Crates:
- [figlet-rs](https://crates.io/crates/figlet-rs)
- [struct-opt](https://docs.rs/structopt/0.3.21/structopt/) for Rust CLI Parsing

## Installation
The following installation instructions are specifically for machines with [Homebrew](https://brew.sh).
```console
brew tap shruthikmusukula/rust-figlet
brew install rust-figlet
```

## Usage
```console
rust-figlet "text in here"
```
- Single word inputs need not be wrapped in quotes, but multi-word inputs require quotes

![Failed to load Demo GIF](https://i.imgur.com/dBcqjMo.gif)
  
## Additional Info
The Homebrew Formula for this project can be found [here](https://github.com/shruthikmusukula/homebrew-rust-figlet).
