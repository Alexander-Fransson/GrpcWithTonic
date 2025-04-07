use crate::utils::traits::{AsHashMap, StructFieldsAsStrings};
use std::collections::HashMap;
// figure out how to just have to import StructFieldsAsStrings once
// also make a to hashmap function

#[test]
fn get_struct_fields_ok() {
    
    #[derive(StructFieldsAsStrings)]    
    struct Foo {
        _a: i32,
        _b: i32,
        _c: i32
    }

    assert_eq!(Foo::get_struct_fields(), vec!["_a", "_b", "_c"]);
}

#[test]
fn to_hashmap_ok() {

    #[derive(AsHashMap, Default)]
    struct Foo {
        _a: i32,
        _b: i32,
        _c: i32
    }

    let instance = Foo::default();

    let map = instance.to_hashmap();

    assert_eq!(map.len(), 3);
    assert_eq!(map.get("_a"), Some(&"0".to_string())); 
    
}