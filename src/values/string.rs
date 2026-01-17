use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::bool::ValueBool;
use crate::values::value::Value;

#[derive(Clone)]
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

    pub fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(i8) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), i8.get_value().unwrap())))),
            Value::I16(i16) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), i16.get_value().unwrap())))),
            Value::I32(i32) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), i32.get_value().unwrap())))),
            Value::I64(i64) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), i64.get_value().unwrap())))),
            Value::U8(u8) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), u8.get_value().unwrap())))),
            Value::U16(u16) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), u16.get_value().unwrap())))),
            Value::U32(u32) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), u32.get_value().unwrap())))),
            Value::U64(u64) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), u64.get_value().unwrap())))),

            Value::F32(f32) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), f32.get_value().unwrap())))),
            Value::F64(f64) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), f64.get_value().unwrap())))),
            Value::Bool(bool) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), bool.get_value().unwrap())))),

            Value::Char(char) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), char.get_value().unwrap())))),
            Value::String(string) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), string.get_value().as_ref().unwrap())))),

            Value::Error(error) => {
                 Some(if let Some(details) = error.get_details() {
                     Value::String(ValueString::init_with(format!("{}{}: {details}", self.value.as_ref().unwrap(), error.get_name().as_ref().unwrap())))
                 } else {
                     Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), error.get_name().as_ref().unwrap())))
                 })
            },
            Value::Type(t) => Some(Value::String(ValueString::init_with(format!("{}{}", self.value.as_ref().unwrap(), t.to_string())))),

            _ => None,
        }
    }

    pub fn not(&self) -> Value {
        let value = match &self.value {
            Some(v) => v.is_empty(),
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
                ApicaTypeBytecode::Any => Some(Value::String(ValueString::init_with(value.clone()))),
                
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(value.clone()))),
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::String(ValueString::init_empty())),
                
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),
                
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecodes::ApicaTypeBytecode;
    use crate::values::string::ValueString;
    use crate::values::value::Value;

    #[test]
    fn test_convert() {
        let string = Value::String(ValueString::init_with(String::from("Hello, world!")));

        let incorrect = string.convert(ApicaTypeBytecode::I8);
        assert_eq!(true, incorrect.is_none());

        let new_string = string.convert(ApicaTypeBytecode::String).unwrap();
        if let Value::String(str) = new_string {
            assert_eq!("Hello, world!", str.get_value().as_ref().unwrap());
        } else {
            panic!("Should be converted to a string");
        }

        let new_type = string.convert(ApicaTypeBytecode::Type).unwrap();
        if let Value::Type(t) = new_type {
            assert_eq!(ApicaTypeBytecode::String, t.get_kind().unwrap());
            assert_eq!(true, t.get_contained().is_none());
        } else {
            panic!("Should be converted to a type");
        }

        let new_bool = string.convert(ApicaTypeBytecode::Bool).unwrap();
        if let Value::Bool(bool) = new_bool {
            assert_eq!(true, bool.get_value().unwrap());
        } else {
            panic!("Should be converted to a bool");
        }
    }
}