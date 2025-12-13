#[derive(PartialEq, Debug)]
pub enum ValueKind {
    Null,
    ElementPointer,
    Any,

    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,

    F32,
    F64,
    Bool,

    Char,
    String,

    Error,
    Type,
}

pub trait Value {
    fn get_kind(&self) -> ValueKind;
    fn show(&self, end: char);
    fn is_null(&self) -> bool;
    fn get_type_representation(&self) -> &str;
}