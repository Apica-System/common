use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::any::ValueAny;
use crate::values::bool::ValueBool;
use crate::values::f32::ValueF32;
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

pub struct ValueChar {
    value: Option<char>,
}

impl ValueChar {
    pub fn init_empty() -> ValueChar {
        return ValueChar { value: None };
    }

    pub fn init_with(value: char) -> ValueChar {
        return ValueChar { value: Some(value) };
    }

    pub fn show(&self, end: char) {
        if let Some(val) = self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        return self.value.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "char";
    }
    
    pub fn get_value(&self) -> Option<char> {
        return self.value;
    }

    pub fn convert(&'_ self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::init_with(*value as u32 as f32))),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::init_with(*value as u32 as f64))),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_with(*value != '\0'))),

                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_with(format!("{value}")))),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::Char, None))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::F32 => Some(Value::F32(ValueF32::init_empty())),
                ApicaTypeBytecode::F64 => Some(Value::F64(ValueF64::init_empty())),
                ApicaTypeBytecode::Bool => Some(Value::Bool(ValueBool::init_empty())),

                ApicaTypeBytecode::String => Some(Value::String(ValueString::init_empty())),
                ApicaTypeBytecode::Type => Some(Value::Type(ValueType::init_with(ApicaTypeBytecode::Char, None))),

                _ => None,
            }
        }
    }

    pub fn auto_convert(&'_ self, to: ApicaTypeBytecode) -> Option<Value> {
        return if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_with(Value::Char(ValueChar::init_with(*value))))
                )),

                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::init_with(*value as i8))),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::init_with(*value as i16))),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::init_with(*value as i32))),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::init_with(*value as i64))),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::init_with(*value as u8))),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::init_with(*value as u16))),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::init_with(*value as u32))),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::init_with(*value as u64))),

                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::init_with(*value))),

                _ => None,
            }
        } else {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Any(
                    Box::new(ValueAny::init_empty())
                )),

                ApicaTypeBytecode::I8 => Some(Value::I8(ValueI8::init_empty())),
                ApicaTypeBytecode::I16 => Some(Value::I16(ValueI16::init_empty())),
                ApicaTypeBytecode::I32 => Some(Value::I32(ValueI32::init_empty())),
                ApicaTypeBytecode::I64 => Some(Value::I64(ValueI64::init_empty())),
                ApicaTypeBytecode::U8 => Some(Value::U8(ValueU8::init_empty())),
                ApicaTypeBytecode::U16 => Some(Value::U16(ValueU16::init_empty())),
                ApicaTypeBytecode::U32 => Some(Value::U32(ValueU32::init_empty())),
                ApicaTypeBytecode::U64 => Some(Value::U64(ValueU64::init_empty())),

                ApicaTypeBytecode::Char => Some(Value::Char(ValueChar::init_empty())),

                _ => None,
            }
        }
    }
}