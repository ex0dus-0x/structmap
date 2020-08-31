# structmap

Procedural macro library for converting between `struct` types and Rust associative containers.

## Introduction

One concern that I've repeatedly come upon when writing Rust is the lack of operability between converting highly structured types, and more "raw" map structures.
This may be the case for instances where we want to represent structured data in a more tabulated format, or the other way around, without resorting to long conditional pattern matching:

```rust
// converting between a struct like ...
struct SomeData {
	key: String
}

// ... and an associative map type like ...
let somedata_hm: HashMap<String, String> = HashMap::new();
```

Using metaprogramming and code generation capabilities  supported by Rust's support for _procedural macros_, I decided to implement this crate to simply provide conversion support between structs (abstractly known as _product types_), and associative containers, including `HashMap`s and `BTreeMap`s (TODO).

This was largely inspired by previous work done by @ameo, linked in his blog [here](https://cprimozic.net/blog/writing-a-hashmap-to-struct-procedural-macro-in-rust/).
This crate contains code that is supported for Rust 2018, and includes updated dependencies for AST parsing and code generation.

## Usage

In your `Cargo.toml` file, include the crate as so:

```
[dependencies]
structmap = "0.1.0"
```

### Map to Struct

Let's demonstrate the conversion from `HashMap` to a `struct` type using the `FromHashMap` derive macro,
where the keys will end up as the attribute names assigned to the values. Note that your `struct` type should extend the `Default` trait
for type conversion to account for uninitialized attributes.

```rust
use structmap::FromHashMap;

#[derive(FromHashMap, Debug)]
struct TestStruct {
    name: String,
    value: String,
}

impl Default for TestStruct {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }
}

fn main() {
	// create a hashmap with key-value pairs
    let mut hm = HashMap::new();
    hm.insert(String::from("name"), String::from("example"));
    hm.insert(String::from("value"), String::from("some_value"));

    // convert hashmap to struct, and check attributes
    let test: TestStruct = TestStruct::from_hashmap(hm);
    assert!(test.name == "example");
    assert!(test.value == "some_value");
}
```

### Map to Struct

TODO

## Contributions

This is still a WIP crate, and will mostly be used for personal projects, but feel free to let me
know if there are any outstanding features that should be implemented!

## License

[MIT License](https://codemuch.tech/license.txt)
