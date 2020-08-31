//! Defines a set of unit tests in order to test the conversion functionality between struct types
//! and associative containers.

use std::collections::HashMap;

// TODO: figure out why I can't reimport from `structmap`


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
    let mut hm: HashMap<String, String> = TestStruct::to_hashmap(TestStruct {
        name: String::from("example"),
        value: String::from("some_value"),
    });
}
