# structmap

Procedural macro library for converting between `struct` types and Rust associative containers.

## Introduction

One concern that I've repeatedly come upon when writing Rust is the lack of operability between converting highly structured types, and more "raw" map structures. This may be the case where, ... without resorting to long conditional pattern matching.

Using metaprogramming supported by Rust's support for _procedural macros_, I decided to implement this crate to simply provide conversion support between structs (abstractly known as _product types_), and associative containers, including `HashMap`s and `BTreeMap`s.

This was largely inspired by previous work done by @ameo, linked in his blog [here](https://cprimozic.net/blog/writing-a-hashmap-to-struct-procedural-macro-in-rust/). This crate contains code that is supported for Rust 2018, and includes updated dependencies for AST parsing and code generation.

## Usage

### Struct to Map

```rust
struct

```

### Map to Struct
