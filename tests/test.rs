//! Defines a set of unit tests in order to test the conversion functionality between struct types
//! and associative containers.
use structmap::{FromMap, ToMap};
use structmap_derive::{FromMap, ToMap};

#[derive(FromMap, ToMap)]
struct TestStruct {
    name: String,
    value: i64,
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
fn test_stringmap_to_struct() {
    let mut hm = StringMap::new();
    hm.insert(String::from("name"), String::from("example"));
    hm.insert(String::from("value"), String::from("0"));

    let test: TestStruct = TestStruct::from_stringmap(hm);
    assert!(test.name == "example");
    assert!(test.value == 0);
}

#[test]
fn test_genericmap_to_struct() {
    let mut hm = GenericMap::new();
    hm.insert(String::from("name"), Value::new("example"));
    hm.insert(String::from("value"), Value::new(0i64));

    let test: TestStruct = TestStruct::from_genericmap(hm);
    assert!(test.name == "example");
    assert!(test.value == 0i64);
}

#[test]
fn test_struct_to_stringmap() {
    let test_struct = TestStruct {
        name: String::from("example"),
        value: 0,
    };

    let hm: StringMap = TestStruct::to_stringmap(test_struct);
    assert!(hm.get("name").unwrap() == "example");
    assert!(hm.get("value").unwrap() == "0");
}

#[test]
fn test_struct_to_genericmap() {
    let test_struct = TestStruct {
        name: String::from("example"),
        value: 0,
    };

    let hm = TestStruct::to_genericmap(test_struct);
    assert!(hm.get("name").unwrap().String().unwrap() == "example");
    assert!(hm.get("value").unwrap().i64().unwrap() == 0);
}

#[derive(ToMap)]
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
fn test_map_to_struct_rename() {
    let test_struct = TestStructRename {
        name: String::from("example"),
        value: String::from("some_value"),
    };

    let hm = TestStructRename::to_stringmap(test_struct);
    assert!(hm.get("Full Name").unwrap().to_string() == "example");
    assert!(hm.get("Data").unwrap().to_string() == "some_value");
}
