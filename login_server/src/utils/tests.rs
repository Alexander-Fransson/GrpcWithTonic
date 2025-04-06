use crate::utils::traits::StructFieldsAsStrings;
use proc_macros::StructFieldsAsStrings;

// figure out how to just have to import StructFieldsAsStrings once

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