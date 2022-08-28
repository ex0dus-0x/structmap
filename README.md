# structmap

[![Actions][actions-badge]][actions-url]
[![crates.io version][crates-structmap-badge]][crates-structmap]
[![Docs][docs-badge]][docs.rs]

[actions-badge]: https://github.com/ex0dus-0x/structmap/workflows/CI/badge.svg?branch=master
[actions-url]: https://github.com/ex0dus-0x/structmap/actions

[crates-structmap-badge]: https://img.shields.io/crates/v/structmap.svg
[crates-structmap]: https://crates.io/crates/structmap

[docs-badge]: https://docs.rs/structmap/badge.svg
[docs.rs]: https://docs.rs/structmap

Procedural macro crate for converting between Rust `struct` types and associative containers.

```rust
use std::collections::BTreeMap;
// converting between a struct like ...
struct SomeData {
    key: String
}

// ... and a BTreeMap like ...
let somedata_hm: BTreeMap<String, String> = BTreeMap::new();
```

This removes the need to pattern match on attributes and keys when making a conversion.

This was largely inspired by [previous work](https://cprimozic.net/blog/writing-a-hashmap-to-struct-procedural-macro-in-rust/) done by [@Ameobea](https://github.com/Ameobea),
but extends on it much further to support conversion both ways, generic value types, and Rust 2018 conventions.

## Usage

In your `Cargo.toml` file, include the crate as so:

```toml
[dependencies]
structmap = "0.1"
```

Now let's demonstrate conversion! Note that your `struct` type should extend the `Default` trait for type conversion to account for uninitialized attributes.

__structmap__ supports conversion between two types of map aliases:

1. `StringMap` - Strings for both keys and values. Conversion is supported only one-way at the moment from struct to BTreeMap, but not the other way around.
2. `GenericMap` - Generic [serde](https://docs.serde.rs/serde_json/enum.Value.html)-style `Value`s as values. Conversion is supported both ways, but limited.

### Map to Struct

```rust
use structmap::FromMap;
use structmap_derive::FromMap;

#[derive(FromMap)]
struct TestStruct {
    name: String,
    value: i64,
}

impl Default for TestStruct {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: 0
        }
    }
}

fn main() {
	// create a hashmap with key-value pairs
    let mut hm = GenericMap::new();

    // `Value` is an enum wrapper to support genericized types, to support structs
    // with varying types for their fields.
    hm.insert(String::from("name"), Value::new(String::from("example")));
    hm.insert(String::from("value"), Value::new(0_i64));

    // convert hashmap to struct, and check attributes
    let test: TestStruct = TestStruct::from_genericmap(hm);
    assert!(test.name == "example");
    assert!(test.value == 0);
}
```

### Struct to Map

```rust
use structmap::{ToMap, value::Value};
use structmap_derive::ToMap;
use std::collections::BTreeMap;

#[derive(ToMap, Default)]
struct TestStruct {
    name: String,
    value: i64,
}

// impl Default ...

fn main() {
    let test_struct = TestStruct {
        name: String::from("example"),
        value: 0,
    };

    // convert struct to generic map, and check attributes
    let hm: BTreeMap<String, Value> = TestStruct::to_genericmap(test_struct);
    assert!(hm.get("name").unwrap().String().unwrap() == "example");
    assert!(hm.get("value").unwrap().i64().unwrap() == 0);

    let test_struct = TestStruct {
        name: String::from("example"),
        value: 0,
    };

    // convert struct to string map, and check attributes
    let hm: BTreeMap<String, String> = TestStruct::to_stringmap(test_struct);
    assert!(hm.get("name").unwrap() == "example");
    assert!(hm.get("value").unwrap() == "0");
}
```

Need a different key name when converting from a `struct` to a map container? Use `#[rename]` for
struct attributes!

```rust
use structmap::ToMap;
use structmap_derive::ToMap;

#[derive(ToMap, Default)]
struct TestStruct {
    #[rename(name = "Full Name")]
    name: String,

    #[rename(name = "Data")]
    value: String,
}
```

## Contributions

All complex types, include dynamic arrays, `Option`s, `Result`s and data structures are not yet
supported (which you can help implement!).

Feel free to let me know if there are any outstanding features that should be implemented!

## License

[MIT License](https://codemuch.tech/license.txt)
