use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::f64::ValueF64;
use crate::values::i16::ValueI16;
use crate::values::i32::ValueI32;
use crate::values::i64::ValueI64;
use crate::values::i8::ValueI8;
use crate::values::string::ValueString;
use crate::values::u16::ValueU16;
use crate::values::u32::ValueU32;
use crate::values::u64::ValueU64;
use crate::values::u8::ValueU8;
use crate::values::value::Value;

pub struct ValueF32 {
    value: Option<f32>,
}

impl ValueF32 {
    pub fn init_empty() -> ValueF32 {
        ValueF32 { value: None }
    }
    
    pub fn init_with(value: f32) -> ValueF32 {
        ValueF32 { value: Some(value) }
    }
    
    pub fn show(&self, end: char) {
        if let Some(val) = self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
    
    pub fn get_type_representation(&self) -> &str {
        "f32"
    }
    
    pub fn get_value(&self) -> Option<f32> {
        self.value
    }

    pub fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(i8) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + i8.get_value().unwrap() as f32))),
            Value::I16(i16) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + i16.get_value().unwrap() as f32))),
            Value::I32(i32) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + i32.get_value().unwrap() as f32))),
            Value::I64(i64) => Some(Value::F64(ValueF64::init_with(self.value.unwrap() as f64 + i64.get_value().unwrap() as f64))),
            Value::U8(u8) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + u8.get_value().unwrap() as f32))),
            Value::U16(u16) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + u16.get_value().unwrap() as f32))),
            Value::U32(u32) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + u32.get_value().unwrap() as f32))),
            Value::U64(u64) => Some(Value::F64(ValueF64::init_with(self.value.unwrap() as f64 + u64.get_value().unwrap() as f64))),

            Value::F32(f32) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + f32.get_value().unwrap()))),
            Value::F64(f64) => Some(Value::F64(ValueF64::init_with(self.value.unwrap() as f64 + f64.get_value().unwrap()))),
            Value::Bool(bool) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + if bool.get_value().unwrap() { 1.0 } else { 0.0 }))),

            Value::Char(char) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() + char.get_value().unwrap() as u32 as f32))),

            _ => None,
        }
    }
    
    pub fn increment(&mut self) -> Option<Value> {
        if let Some(value) = self.value.as_mut() {
            let old_value = Value::F32(ValueF32::init_with(*value));
            *value += 1.0;
            Some(old_value)
        } else {
            unreachable!()
        }
    }
    
    pub fn decrement(&mut self) -> Option<Value> {
        if let Some(value) = self.value.as_mut() {
            let old_value = Value::F32(ValueF32::init_with(*value));
            *value -= 1.0;
            Some(old_value)
        } else {
            unreachable!()
        }
    }
    
    pub fn not(&self) -> Value {
        let value = match self.value {
            Some(v) => v == 0.0,
            None => true,
        };
        Value::Bool(ValueBool::init_with(value))
    }
    
    pub fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Char =>
                    if let Some(converted) = char::from_u32(*value as u32) {
                        Some(Value::Char(ValueChar::init_with(converted)))
                    } else {
                        Some(Value::Char(ValueChar::init_with('�')))
                    },
                
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(format!("{value}")))),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::F32, None))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::init_empty())),
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::F32, None))),

                _ => None,
            }
        }
    }
    
    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Any => Some(Value::F32(ValueF32::init_with(*value))),
                
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::init_with(*value as i8))),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::init_with(*value as i16))),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::init_with(*value as i32))),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::init_with(*value as i64))),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::init_with(*value as u8))),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::init_with(*value as u16))),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::init_with(*value as u32))),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::init_with(*value as u64))),
                ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::init_with(*value))),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::init_with(*value as f64))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_with(*value != 0.0))),
                
                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::F32(ValueF32::init_empty())),
                
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::init_empty())),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::init_empty())),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::init_empty())),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::init_empty())),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::init_empty())),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::init_empty())),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::init_empty())),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::init_empty())),
                ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::init_empty())),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::init_empty())),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_empty())),
                
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::values::char::ValueChar;
    use crate::values::f32::ValueF32;
    use crate::values::i16::ValueI16;
    use crate::values::string::ValueString;
    use crate::values::u64::ValueU64;
    use crate::values::value::Value;

    #[test]
    fn test_empty() {
        let mut f32 = ValueF32::init_empty();
        assert!(f32.is_null());

        f32 = ValueF32::init_with(12.0);
        assert!(!f32.is_null());
    }

    #[test]
    fn test_type_repr() {
        let f32 = ValueF32::init_empty();
        assert_eq!("f32", f32.get_type_representation());
    }

    #[test]
    fn test_get_value() {
        let mut f32 = ValueF32::init_empty();
        assert!(f32.get_value().is_none());

        f32 = ValueF32::init_with(12.0);
        assert_eq!(12.0, f32.get_value().unwrap());
    }

    #[test]
    fn test_add() {
        let f32 = ValueF32::init_with(12.0);

        let i16 = Value::I16(ValueI16::init_with(-12));
        let mut result = f32.add(&i16);
        if let Value::F32(i16_result) = &result.unwrap() {
            assert_eq!(0.0, i16_result.get_value().unwrap());
        } else { panic!(); }

        let u64 = Value::U64(ValueU64::init_with(10));
        result = f32.add(&u64);
        if let Value::F64(u64_result) = &result.unwrap() {
            assert_eq!(22.0, u64_result.get_value().unwrap());
        } else { panic!(); }

        let other_f32 = Value::F32(ValueF32::init_with(-12.0));
        result = f32.add(&other_f32);
        if let Value::F32(float_result) = &result.unwrap() {
            assert_eq!(0.0, float_result.get_value().unwrap());
        } else { panic!(); }

        let char = Value::Char(ValueChar::init_with('a'));
        result = f32.add(&char);
        if let Value::F32(char_result) = &result.unwrap() {
            assert_eq!(109.0, char_result.get_value().unwrap());
        } else { panic!(); }

        let string = Value::String(ValueString::init_with(String::new()));
        result = f32.add(&string);
        assert!(result.is_none());
    }

    #[test]
    fn test_increment() {
        let mut f32 = ValueF32::init_with(12.0);
        let result = f32.increment();
        if let Value::F32(v) = &result.unwrap() {
            assert_eq!(12.0, v.get_value().unwrap());
        } else { panic!(); }

        assert_eq!(13.0, f32.get_value().unwrap());
    }

    #[test]
    fn test_decrement() {
        let mut f32 = ValueF32::init_with(12.0);
        let result = f32.decrement();
        if let Value::F32(v) = &result.unwrap() {
            assert_eq!(12.0, v.get_value().unwrap());
        }

        assert_eq!(11.0, f32.get_value().unwrap());
    }

    #[test]
    fn test_not() {
        let mut f32 = ValueF32::init_empty();
        let mut result = f32.not();
        if let Value::Bool(bool_result) = &result {
            assert_eq!(true, bool_result.get_value().unwrap());
        } else { panic!(); }

        f32 = ValueF32::init_with(0.0);
        result = f32.not();
        if let Value::Bool(bool_result) = &result {
            assert_eq!(true, bool_result.get_value().unwrap());
        } else { panic!(); }

        f32 = ValueF32::init_with(1.0);
        result = f32.not();
        if let Value::Bool(bool_result) = &result {
            assert_eq!(false, bool_result.get_value().unwrap());
        } else { panic!(); }
    }
}