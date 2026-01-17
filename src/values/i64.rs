use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::f32::ValueF32;
use crate::values::f64::ValueF64;
use crate::values::i16::ValueI16;
use crate::values::i32::ValueI32;
use crate::values::i8::ValueI8;
use crate::values::string::ValueString;
use crate::values::u16::ValueU16;
use crate::values::u32::ValueU32;
use crate::values::u64::ValueU64;
use crate::values::u8::ValueU8;
use crate::values::value::Value;

#[derive(Clone)]
pub struct ValueI64 {
    value: Option<i64>,
}

impl ValueI64 {
    pub fn init_empty() -> ValueI64 {
        ValueI64 { value: None }
    }
    
    pub fn init_with(value: i64) -> ValueI64 {
        ValueI64 { value: Some(value) }
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
        "i64"
    }
    
    pub fn get_value(&self) -> Option<i64> {
        self.value
    }

    pub fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(i8) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + i8.get_value().unwrap() as i64))),
            Value::I16(i16) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + i16.get_value().unwrap() as i64))),
            Value::I32(i32) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + i32.get_value().unwrap() as i64))),
            Value::I64(i64) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + i64.get_value().unwrap()))),
            Value::U8(u8) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + u8.get_value().unwrap() as i64))),
            Value::U16(u16) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + u16.get_value().unwrap() as i64))),
            Value::U32(u32) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + u32.get_value().unwrap() as i64))),
            Value::U64(u64) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + u64.get_value().unwrap() as i64))),

            Value::F32(f32) => Some(Value::F64(ValueF64::init_with(self.value.unwrap() as f64 + f32.get_value().unwrap() as f64))),
            Value::F64(f64) => Some(Value::F64(ValueF64::init_with(self.value.unwrap() as f64 + f64.get_value().unwrap()))),
            Value::Bool(bool) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + if bool.get_value().unwrap() { 1 } else { 0 }))),

            Value::Char(char) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() + char.get_value().unwrap() as i64))),

            _ => None,
        }
    }
    
    pub fn increment(&mut self) -> Option<Value> {
        if let Some(value) = self.value.as_mut() {
            let old_value = Value::I64(ValueI64::init_with(*value));
            *value += 1;
            Some(old_value)
        } else {
            unreachable!()
        }
    }
    
    pub fn decrement(&mut self) -> Option<Value> {
        if let Some(value) = self.value.as_mut() {
            let old_value = Value::I64(ValueI64::init_with(*value));
            *value -= 1;
            Some(old_value)
        } else {
            unreachable!()
        }
    }

    pub fn not(&self) -> Value {
        let value = match self.value {
            Some(v) => v == 0,
            None => true,
        };
        Value::Bool(ValueBool::init_with(value))
    }
    
    pub fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(format!("{value}")))),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::I64, None))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::I64, None))),

                _ => None,
            }
        }
    }

    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Any => Some(Value::I64(ValueI64::init_with(*value))),
                
                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::init_with(*value as i8))),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::init_with(*value as i16))),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::init_with(*value as i32))),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::init_with(*value))),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::init_with(*value as u8))),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::init_with(*value as u16))),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::init_with(*value as u32))),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::init_with(*value as u64))),
                ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::init_with(*value as f32))),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::init_with(*value as f64))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_with(*value != 0))),
                ApicaTypeBytecode::Char => Some(ValueChar::from_u32(*value as u32)),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::I64(ValueI64::init_empty())),
                
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
                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::init_empty())),

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
    use crate::values::i64::ValueI64;
    use crate::values::string::ValueString;
    use crate::values::u64::ValueU64;
    use crate::values::value::Value;

    #[test]
    fn test_empty() {
        let mut i64 = ValueI64::init_empty();
        assert!(i64.is_null());

        i64 = ValueI64::init_with(12);
        assert!(!i64.is_null());
    }

    #[test]
    fn test_type_repr() {
        let i64 = ValueI64::init_empty();
        assert_eq!("i64", i64.get_type_representation());
    }

    #[test]
    fn test_get_value() {
        let mut i64 = ValueI64::init_empty();
        assert!(i64.get_value().is_none());

        i64 = ValueI64::init_with(12);
        assert_eq!(12, i64.get_value().unwrap());
    }

    #[test]
    fn test_add() {
        let i64 = ValueI64::init_with(12);

        let i16 = Value::I16(ValueI16::init_with(-12));
        let mut result = i64.add(&i16);
        if let Value::I64(i16_result) = &result.unwrap() {
            assert_eq!(0, i16_result.get_value().unwrap());
        } else { panic!(); }

        let u64 = Value::U64(ValueU64::init_with(10));
        result = i64.add(&u64);
        if let Value::I64(u64_result) = &result.unwrap() {
            assert_eq!(22, u64_result.get_value().unwrap());
        } else { panic!(); }

        let f32 = Value::F32(ValueF32::init_with(-12.0));
        result = i64.add(&f32);
        if let Value::F64(float_result) = &result.unwrap() {
            assert_eq!(0.0, float_result.get_value().unwrap());
        } else { panic!(); }

        let char = Value::Char(ValueChar::init_with('a'));
        result = i64.add(&char);
        if let Value::I64(char_result) = &result.unwrap() {
            assert_eq!(109, char_result.get_value().unwrap());
        } else { panic!(); }

        let string = Value::String(ValueString::init_with(String::new()));
        result = i64.add(&string);
        assert!(result.is_none());
    }

    #[test]
    fn test_increment() {
        let mut i64 = ValueI64::init_with(12);
        let result = i64.increment();
        if let Value::I64(v) = &result.unwrap() {
            assert_eq!(12, v.get_value().unwrap());
        } else { panic!(); }

        assert_eq!(13, i64.get_value().unwrap());
    }

    #[test]
    fn test_decrement() {
        let mut i64 = ValueI64::init_with(12);
        let result = i64.decrement();
        if let Value::I64(v) = &result.unwrap() {
            assert_eq!(12, v.get_value().unwrap());
        }

        assert_eq!(11, i64.get_value().unwrap());
    }

    #[test]
    fn test_not() {
        let mut i64 = ValueI64::init_empty();
        let mut result = i64.not();
        if let Value::Bool(bool_result) = &result {
            assert_eq!(true, bool_result.get_value().unwrap());
        } else { panic!(); }

        i64 = ValueI64::init_with(0);
        result = i64.not();
        if let Value::Bool(bool_result) = &result {
            assert_eq!(true, bool_result.get_value().unwrap());
        } else { panic!(); }

        i64 = ValueI64::init_with(1);
        result = i64.not();
        if let Value::Bool(bool_result) = &result {
            assert_eq!(false, bool_result.get_value().unwrap());
        } else { panic!(); }
    }
}