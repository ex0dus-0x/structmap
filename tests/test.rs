//! Defines a set of unit tests in order to test the conversion functionality between struct types
//! and associative containers.

use structmap::FromHashMap;

use std::collections::HashMap;

/*
#[structmap::to_hashmap]
type DeriveHashMap = HashMap<String, String>;
*/

#[derive(FromHashMap)]
struct TestStruct {
    name: String,
    value: String
}


#[test]
fn test_hashmap_to_struct() {
    let mut hm = HashMap::new();
    hm.insert(String::from("name"), String::from("example"));
    hm.insert(String::from("value"), String::from("some_value"));

    let test: TestStruct = TestStruct::from_hashmap(hm);
}

/*
#[test]
fn test_struct_to_hashmap() {
    let mut hm: DeriveHashMap = TestStruct::to_hashmap(TestStruct {
        name: String::from("example"),
        value: String::from("some_value"),
    });
}
*/