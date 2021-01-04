//! Defines a set of unit tests in order to test the conversion functionality between struct types
//! and associative containers.

use std::collections::HashMap;

use structmap::{FromHashMap, ToHashMap};
use structmap_derive::{FromHashMap, ToHashMap};

#[derive(FromHashMap, ToHashMap)]
struct TestStruct {
    name: String,
    value: i32,
}

impl Default for TestStruct {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: 0,
        }
    }
}

#[test]
fn test_hashmap_to_struct() {
    // first create a HashMap mapping identifier names to generic Value type
    let mut hm: HashMap<String, Value> = HashMap::new();
    hm.insert(String::from("name"), Value::new("example"));
    hm.insert(String::from("value"), Value::new(0));

    // convert hashmap to struct, and check attributes
    let test: TestStruct = TestStruct::from_hashmap(hm);
    assert!(test.name == "example");
    assert!(test.value == 0);
}

#[test]
fn test_struct_to_hashmap() {
    let test_struct = TestStruct {
        name: String::from("example"),
        value: 0,
    };

    // convert struct to hashmap, and check attributes
    let hm: HashMap<String, Value> = TestStruct::to_hashmap(test_struct);
    assert!(hm.get("name").unwrap().to_string().unwrap() == "example");
    assert!(hm.get("value").unwrap().to_i32().unwrap() == 0);
}

#[derive(ToHashMap)]
struct TestStructRename {
    #[rename(name = "Full Name")]
    name: String,

    #[rename(name = "Data")]
    value: String,
}

impl Default for TestStructRename {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
        }
    }
}

#[test]
fn test_hm_to_struct_rename() {
    let test_struct = TestStructRename {
        name: String::from("example"),
        value: String::from("some_value"),
    };

    let hm: HashMap<String, Value> = TestStructRename::to_hashmap(test_struct);
    assert!(hm.get("Full Name").unwrap().to_string().unwrap() == "example");
    assert!(hm.get("Data").unwrap().to_string().unwrap() == "some_value");
}
