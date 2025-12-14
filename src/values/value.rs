use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::any::ValueAny;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::element_pointer::ValueElementPointer;
use crate::values::error::ValueError;
use crate::values::f32::ValueF32;
use crate::values::f64::ValueF64;
use crate::values::i16::ValueI16;
use crate::values::i32::ValueI32;
use crate::values::i64::ValueI64;
use crate::values::i8::ValueI8;
use crate::values::null::ValueNull;
use crate::values::string::ValueString;
use crate::values::u16::ValueU16;
use crate::values::u32::ValueU32;
use crate::values::u64::ValueU64;
use crate::values::u8::ValueU8;

pub enum Value {
    Null(ValueNull),
    ElementPointer(Box<ValueElementPointer>),
    Any(Box<ValueAny>),
    
    I8(ValueI8),
    I16(ValueI16),
    I32(ValueI32),
    I64(ValueI64),
    U8(ValueU8),
    U16(ValueU16),
    U32(ValueU32),
    U64(ValueU64),
    
    F32(ValueF32),
    F64(ValueF64),
    Bool(ValueBool),
    
    Char(ValueChar),
    String(ValueString),
    
    Error(ValueError),
    Type(ValueType),
}

impl Value {
    pub fn get_kind(&self) -> ApicaTypeBytecode {
        return match self { 
            Value::Null(_) => ApicaTypeBytecode::Null,
            Value::ElementPointer(element_pointer) => element_pointer.get_value().get_value().get_kind(),
            Value::Any(any) => 
                if let Some(val) = any.get_value() {
                    val.get_kind()
                } else {
                    ApicaTypeBytecode::Null
                },
            
            Value::I8(_) => ApicaTypeBytecode::I8,
            Value::I16(_) => ApicaTypeBytecode::I16,
            Value::I32(_) => ApicaTypeBytecode::I32,
            Value::I64(_) => ApicaTypeBytecode::I64,
            Value::U8(_) => ApicaTypeBytecode::U8,
            Value::U16(_) => ApicaTypeBytecode::U16,
            Value::U32(_) => ApicaTypeBytecode::U32,
            Value::U64(_) => ApicaTypeBytecode::U64,
            
            Value::F32(_) => ApicaTypeBytecode::F32,
            Value::F64(_) => ApicaTypeBytecode::F64,
            Value::Bool(_) => ApicaTypeBytecode::Bool,
            
            Value::Char(_) => ApicaTypeBytecode::Char,
            Value::String(_) => ApicaTypeBytecode::String,
            
            Value::Error(_) => ApicaTypeBytecode::Error,
            Value::Type(_) => ApicaTypeBytecode::Type,
        }
    }
    
    pub fn show(&self, end: char) {
        match self {
            Value::Null(null) => null.show(end),
            Value::ElementPointer(element_pointer) => element_pointer.show(end),
            Value::Any(any) => any.show(end),
            
            Value::I8(i8) => i8.show(end),
            Value::I16(i16) => i16.show(end),
            Value::I32(i32) => i32.show(end),
            Value::I64(i64) => i64.show(end),
            Value::U8(u8) => u8.show(end),
            Value::U16(u16) => u16.show(end),
            Value::U32(u32) => u32.show(end),
            Value::U64(u64) => u64.show(end),
            
            Value::F32(f32) => f32.show(end),
            Value::F64(f64) => f64.show(end),
            Value::Bool(bool) => bool.show(end),
            
            Value::Char(char) => char.show(end),
            Value::String(string) => string.show(end),
            
            Value::Error(error) => error.show(end),
            Value::Type(t) => t.show(end),
        }
    }
    
    pub fn is_null(&self) -> bool {
        return match self {
            Value::Null(null) => null.is_null(),
            Value::ElementPointer(element_pointer) => element_pointer.is_null(),
            Value::Any(any) => any.is_null(),
            
            Value::I8(i8) => i8.is_null(),
            Value::I16(i16) => i16.is_null(),
            Value::I32(i32) => i32.is_null(),
            Value::I64(i64) => i64.is_null(),
            Value::U8(u8) => u8.is_null(),
            Value::U16(u16) => u16.is_null(),
            Value::U32(u32) => u32.is_null(),
            Value::U64(u64) => u64.is_null(),
            
            Value::F32(f32) => f32.is_null(),
            Value::F64(f64) => f64.is_null(),
            Value::Bool(bool) => bool.is_null(),
            
            Value::Char(char) => char.is_null(),
            Value::String(string) => string.is_null(),
            
            Value::Error(error) => error.is_null(),
            Value::Type(t) => t.is_null(),
        }
    }
    
    pub fn get_type_representation(&self) -> &str {
        return match self {
            Value::Null(null) => null.get_type_representation(),
            Value::ElementPointer(element_pointer) => element_pointer.get_type_representation(),
            Value::Any(any) => any.get_type_representation(),
            
            Value::I8(i8) => i8.get_type_representation(),
            Value::I16(i16) => i16.get_type_representation(),
            Value::I32(i32) => i32.get_type_representation(),
            Value::I64(i64) => i64.get_type_representation(),
            Value::U8(u8) => u8.get_type_representation(),
            Value::U16(u16) => u16.get_type_representation(),
            Value::U32(u32) => u32.get_type_representation(),
            Value::U64(u64) => u64.get_type_representation(),
            
            Value::F32(f32) => f32.get_type_representation(),
            Value::F64(f64) => f64.get_type_representation(),
            Value::Bool(bool) => bool.get_type_representation(),
            
            Value::Char(char) => char.get_type_representation(),
            Value::String(string) => string.get_type_representation(),
            
            Value::Error(error) => error.get_type_representation(),
            Value::Type(t) => t.get_type_representation(),
        }
    }
}