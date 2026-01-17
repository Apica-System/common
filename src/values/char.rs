use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
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

#[derive(Clone)]
pub struct ValueChar {
    value: Option<char>,
}

impl ValueChar {
    pub fn init_empty() -> ValueChar {
        ValueChar { value: None }
    }

    pub fn init_with(value: char) -> ValueChar {
        ValueChar { value: Some(value) }
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
        "char"
    }
    
    pub fn get_value(&self) -> Option<char> {
        self.value
    }

    pub fn from_u32(value: u32) -> Value {
        if let Some(character) = char::from_u32(value) {
            Value::Char(ValueChar::init_with(character))
        } else {
            Value::Char(ValueChar::init_with('�'))
        }
    }

    pub fn add(&self, other: &Value) -> Option<Value> {
        match other {
            Value::I8(i8) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + i8.get_value().unwrap() as u32)),
            Value::I16(i16) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + i16.get_value().unwrap() as u32)),
            Value::I32(i32) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + i32.get_value().unwrap() as u32)),
            Value::I64(i64) => Some(Value::I64(ValueI64::init_with(self.value.unwrap() as i64 + i64.get_value().unwrap()))),
            Value::U8(u8) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + u8.get_value().unwrap() as u32)),
            Value::U16(u16) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + u16.get_value().unwrap() as u32)),
            Value::U32(u32) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + u32.get_value().unwrap())),
            Value::U64(u64) => Some(Value::U64(ValueU64::init_with(self.value.unwrap() as u64 + u64.get_value().unwrap()))),

            Value::F32(f32) => Some(Value::F32(ValueF32::init_with(self.value.unwrap() as u32 as f32 + f32.get_value().unwrap()))),
            Value::F64(f64) => Some(Value::F64(ValueF64::init_with(self.value.unwrap() as u32 as f64 + f64.get_value().unwrap()))),
            Value::Bool(bool) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + if bool.get_value().unwrap() { 1 } else { 0 })),

            Value::Char(char) => Some(ValueChar::from_u32(self.value.unwrap() as u32 + char.get_value().unwrap() as u32)),

            _ => None,
        }
    }

    pub fn not(&self) -> Value {
        let value = match self.value {
            Some(value) => value == '\0',
            None => true,
        };
        Value::Bool(ValueBool::init_with(value))
    }
    
    pub fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
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

    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(value) = &self.value {
            match to {
                ApicaTypeBytecode::Any => Some(Value::Char(ValueChar::init_with(*value))),
                
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
                ApicaTypeBytecode::Any => Some(Value::Char(ValueChar::init_empty())),
                
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