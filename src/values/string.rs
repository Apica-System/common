use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::bool::ValueBool;
use crate::values::value::Value;

pub struct ValueString {
    value: Option<String>,
}

impl ValueString {
    pub fn init_empty() -> ValueString {
        ValueString { value: None }
    }

    pub fn init_with(value: String) -> ValueString {
        ValueString { value: Some(value) }
    }

    pub fn show(&self, end: char) {
        if let Some(val) = &self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
    
    pub fn get_type_representation(&self) -> &str {
        "string"
    }

    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }
    
    pub fn not(&self) -> Value {
        let value = match &self.value {
            Some(v) => !v.is_empty(),
            None => true,
        };
        Value::Bool(ValueBool::init_with(value))
    }
    
    pub fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
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
    
    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {        
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(value.clone()))),
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),
                
                _ => None,
            }
        }
    }
}