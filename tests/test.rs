//! Defines a set of unit tests in order to test the conversion functionality between struct types
//! and associative containers.

use std::collections::HashMap;

use structmap::{FromHashMap, ToHashMap};
use structmap_derive::{FromHashMap, ToHashMap};


#[derive(FromHashMap, ToHashMap)]
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


#[derive(ToHashMap)]
struct TestStructRename {
    #[rename(name = "Full Name")]
    name: String,

    #[rename(name = "Age")]
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
fn test_hashmap_to_struct() {
    let mut hm = HashMap::new();
    hm.insert(String::from("name"), String::from("example"));
    hm.insert(String::from("value"), String::from("some_value"));

    // convert hashmap to struct, and check attributes
    let test: TestStruct = TestStruct::from_hashmap(hm);
    assert!(test.name == "example");
    assert!(test.value == "some_value");
}

#[test]
fn test_struct_to_hashmap() {
    let test_struct = TestStruct {
        name: String::from("example"),
        value: String::from("some_value"),
    };

    // convert struct to hashmap, and check attributes
    let hm: HashMap<String, String> = TestStruct::to_hashmap(test_struct);
    assert!(hm.get("name").unwrap() == "example");
    assert!(hm.get("value").unwrap() == "some_value");
}
