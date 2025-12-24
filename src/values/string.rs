use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::any::ValueAny;
use crate::values::bool::ValueBool;
use crate::values::value::Value;

pub struct ValueString {
    value: Option<String>,
}

impl ValueString {
    pub fn init_empty() -> ValueString {
        return ValueString { value: None };
    }

    pub fn init_with(value: String) -> ValueString {
        return ValueString { value: Some(value) };
    }

    pub fn show(&self, end: char) {
        if let Some(val) = &self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        return self.value.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "string";
    }
    
    pub fn init_from(value: &str) -> ValueString {
        return ValueString { value: Some(String::from(value)) };
    }

    pub fn get_value(&self) -> &Option<String> {
        return &self.value;
    }
    
    pub fn convert(&'_ self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_with(!value.is_empty()))),
                
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::String, None))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_empty())),

                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::String, None))),

                _ => None,
            }
        }
    }
    
    pub fn auto_convert(&'_ self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_with(Value::String(ValueString::init_with(value.clone()))))
                )),
                
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(value.clone()))),
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_empty())
                )),
                
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),
                
                _ => None,
            }
        }
    }
}