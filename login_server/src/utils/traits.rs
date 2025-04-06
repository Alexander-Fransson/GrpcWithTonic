// proc macros

pub trait StructFieldsAsStrings {
    fn get_struct_fields() -> Vec<&'static str>;
}