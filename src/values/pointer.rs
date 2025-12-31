use crate::values::bool::ValueBool;
use crate::values::value::Value;

pub struct ValuePointer {
    pointer: String,
    is_global: bool,
}

impl ValuePointer {
    pub fn init_with(pointer: String, is_global: bool) -> ValuePointer {
        ValuePointer { pointer, is_global }
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
    
    pub fn is_global(&self) -> bool {
        self.is_global
    }
    
    pub fn not(&self) -> Value {
        Value::Bool(ValueBool::init_with(false))
    }
}