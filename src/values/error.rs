use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::any::ValueAny;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::Value;

pub struct ValueError {
    name: Option<String>,
    details: Option<String>,
}

impl ValueError {
    pub fn init_empty() -> ValueError {
        return ValueError { name: None, details: None };
    }

    pub fn init_with(name: String, details: Option<String>) -> ValueError {
        return ValueError { name: Some(name), details };
    }
    
    pub fn show(&self, end: char) {
        if let Some(name) = &self.name {
            print!("{name}");
            if let Some(details) = &self.details {
                print!(": {details}");
            }
            print!("{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        return self.name.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "error";
    }

    pub fn get_name(&self) -> &Option<String> {
        return &self.name;
    }

    pub fn get_details(&self) -> &Option<String> {
        return &self.details;
    }
    
    pub fn convert(&'_ self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(name) = &self.name {
            match to {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_with(true))),
                
                ApicaTypeBytecode::String =>
                    if let Some(details) = &self.details {
                        Some(Value::String(ValueString::init_with(format!("error<{name}: {details}>"))))
                    } else {
                        Some(Value::String(ValueString::init_with(format!("error<{name}>"))))
                    },

                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::Error, None))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_empty())),
                
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),

                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::Error, None))),

                _ => None,
            }
        }
    }
    
    pub fn auto_convert(&'_ self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(name) = &self.name {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_with(Value::Error(ValueError::init_with(name.clone(), self.details.clone())))),
                )),
                
                ApicaTypeBytecode::Error => Some(Value::Error(ValueError::init_with(name.clone(), self.details.clone()))),
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_empty())
                )),
                
                ApicaTypeBytecode::Error => Some(Value::Error(ValueError::init_empty())),
                
                _ => None,
            }
        }
    }
}