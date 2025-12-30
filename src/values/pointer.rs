use crate::values::bool::ValueBool;
use crate::values::value::Value;

pub struct ValuePointer {
    pointer: String,
}

impl ValuePointer {
    pub fn init_with(pointer: String) -> ValuePointer {
        ValuePointer { pointer }
    }

    pub fn is_null(&self) -> bool {
        true
    }
    
    pub fn get_type_representation(&self) -> &str {
        "elt-pointer"
    }

    pub fn get_pointer(&self) -> &str {
        &self.pointer
    }
    
    pub fn not(&self) -> Value {
        Value::Bool(ValueBool::init_with(false))
    }
}