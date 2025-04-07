// proc macros

use std::collections::HashMap;

pub use proc_macros::StructFieldsAsStrings;
pub trait StructFieldsAsStrings {
    fn get_struct_fields() -> Vec<&'static str>;
}

// pub use the derive macro from the proc-macros crate
// then you don have to both import the trait and the derive macro
pub use proc_macros::AsHashMap;
pub trait AsHashMap {
    fn to_hashmap(&self) -> HashMap<&'static str, String>;
}
