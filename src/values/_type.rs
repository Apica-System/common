use crate::bytecodes::ApicaTypeBytecode;
use crate::values::any::ValueAny;
use crate::values::bool::ValueBool;
use crate::values::string::ValueString;
use crate::values::value::Value;

#[derive(Clone)]
pub struct ValueType {
    kind: Option<ApicaTypeBytecode>,
    contained: Option<Vec<ValueType>>,
}

pub fn get_kind_repr(kind: &ApicaTypeBytecode) -> &'static str {
    return match kind {
        ApicaTypeBytecode::Null => "null",
        ApicaTypeBytecode::Any => "any",
        ApicaTypeBytecode::I8 => "i8",
        ApicaTypeBytecode::I16 => "i16",
        ApicaTypeBytecode::I32 => "i32",
        ApicaTypeBytecode::I64 => "i64",
        ApicaTypeBytecode::U8 => "u8",
        ApicaTypeBytecode::U16 => "u16",
        ApicaTypeBytecode::U32 => "u32",
        ApicaTypeBytecode::U64 => "u64",
        ApicaTypeBytecode::F32 => "f32",
        ApicaTypeBytecode::F64 => "f64",
        ApicaTypeBytecode::Bool => "bool",
        ApicaTypeBytecode::Char => "char",
        ApicaTypeBytecode::String => "string",
        ApicaTypeBytecode::Error => "error",
        ApicaTypeBytecode::Type => "type",
    }
}

impl ValueType {
    pub fn init_empty() -> ValueType {
        return ValueType { kind: None, contained: None };
    }

    pub fn init_with(kind: ApicaTypeBytecode, contained: Option<Vec<ValueType>>) -> ValueType {
        return ValueType { kind: Some(kind), contained };
    }
    
    pub fn show(&self, end: char) {
        if let Some(kind) = &self.kind {
            let repr = get_kind_repr(kind);
            print!("{repr}");
            if let Some(contained) = &self.contained {
                print!("<");
                for i in 0..contained.len() {
                    let c = &contained[i];
                    c.show('\0');
                    if i != contained.len() - 1 {
                        print!(", ");
                    }
                }

                print!(">");
            }
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        return self.kind.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "type";
    }

    pub fn get_kind(&self) -> &Option<ApicaTypeBytecode> {
        return &self.kind;
    }

    pub fn get_contained(&self) -> &Option<Vec<ValueType>> {
        return &self.contained;
    }

    fn to_string(&self) -> String {
        return if let Some(kind) = &self.kind {
            let mut string = format!("{}", get_kind_repr(kind));
            if let Some(contained) = &self.contained {
                string.push('<');
                for c in contained {
                    let additional = c.to_string();
                    if !additional.is_empty() {
                        string.push_str(additional.as_str());
                    }
                }
                string.push('>');
            }

            string
        } else {
            String::new()
        };
    }

    pub fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(_) = &self.kind {
            match to {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_with(true))),

                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(self.to_string()))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_empty())),

                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),

                _ => None,
            }
        }
    }

    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(kind) = &self.kind {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_with(Value::Type(ValueType::init_with(*kind, self.contained.clone()))))
                )),

                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::Type, None))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_empty())
                )),

                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::Type, None))),

                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecodes::ApicaTypeBytecode;
    use crate::values::_type::ValueType;
    use crate::values::value::Value;

    #[test]
    fn test_to_string() {
        let base_type = Value::Type(ValueType::init_with(ApicaTypeBytecode::Type, Some(vec![
            ValueType::init_with(ApicaTypeBytecode::String, None)
        ])));

        let string_type = base_type.convert(ApicaTypeBytecode::String).unwrap();
        if let Value::String(str) = &string_type {
            if let Some(value) = &str.get_value() {
                assert_eq!("type<string>", value);
            }
        } else {
            panic!("Should be a string");
        }
    }
}