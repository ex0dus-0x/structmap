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
// converting between a struct like ...
struct SomeData {
    key: String
}

// ... and a HashMap like ...
let somedata_hm = HashMap::new();
```

This removes the need to pattern match on attributes and keys when making a conversion.

This was largely inspired by [previous work](https://cprimozic.net/blog/writing-a-hashmap-to-struct-procedural-macro-in-rust/) done by [@Ameobea](https://github.com/Ameobea),
but extends on it much further to support conversion both ways, generic value types, and Rust 2018 conventions.

## Usage

In your `Cargo.toml` file, include the crate as so:

```
[dependencies]
structmap = "0.1"
```

Now let's demonstrate conversion! Note that your `struct` type should extend the `Default` trait for type conversion to account for uninitialized attributes.

### Map to Struct

```rust
use structmap::FromHashMap;
use structmap_derive::FromHashMap;

#[derive(FromHashMap)]
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

    // `Value` is an enum wrapper to support genericized types, to support structs
    // with varying types for their fields.
    hm.insert(String::from("name"), Value::new(String::from("example")));
    hm.insert(String::from("value"), Value::new(String::from("some_value")));

    // convert hashmap to struct, and check attributes
    let test: TestStruct = TestStruct::from_hashmap(hm);
    assert!(test.name == "example");
    assert!(test.value == "some_value");
}
```

### Struct to Map

```rust
use structmap::ToHashMap;
use structmap_derive::ToHashMap;

#[derive(ToHashMap)]
struct TestStruct {
    name: String,
    value: String,
}

// impl Default ...

fn main() {
    let test_struct = TestStruct {
        name: String::from("example"),
        value: String::from("some_value"),
    };

    // convert struct to hashmap, and check attributes
    let hm: HashMap<String, Value> = TestStruct::to_hashmap(test_struct);
    assert!(hm.get("name").unwrap().to_string().unwrap() == "example");
    assert!(hm.get("value").unwrap().to_string().unwrap() == "some_value");
}
```

Need a different key name when converting from a `struct` to a map container? Use `#[rename]` for
struct attributes!

```rust
#[derive(ToHashMap)]
struct TestStruct {
    #[rename(name = "Full Name")]
    name: String,

    #[rename(name = "Data")]
    value: String,
}
```

## Limitations

At the current moment, `Value` only supports encapsulate basic primitive types, which includes the
following:

* `i32` and `u32` numerals
* `bool`s
* `String`s and `&str`s

All other types, include dynamic arrays, `Option`s, `Result`s and complex structures are not yet
supported (which you can help implement)

## Contributions

This is still a WIP crate, and will mostly be used for personal projects, but feel free to let me
know if there are any outstanding features that should be implemented!

## License

[MIT License](https://codemuch.tech/license.txt)
