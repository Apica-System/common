use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::error::ValueError;
use crate::values::f32::ValueF32;
use crate::values::f64::ValueF64;
use crate::values::i16::ValueI16;
use crate::values::i32::ValueI32;
use crate::values::i64::ValueI64;
use crate::values::i8::ValueI8;
use crate::values::null::ValueNull;
use crate::values::pointer::ValuePointer;
use crate::values::string::ValueString;
use crate::values::u16::ValueU16;
use crate::values::u32::ValueU32;
use crate::values::u64::ValueU64;
use crate::values::u8::ValueU8;

pub enum Value {
    Null(ValueNull),
    Pointer(ValuePointer),

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
    pub fn unary_operation_error(op: &str, operand: &str) -> Value {
        Value::Error(ValueError::init_with(
            String::from("OperationError"),
            Some(format!("Unary operator `{op}` is not defined for type <{operand}>")),
        ))
    }
    
    pub fn binary_operation_error(op: &str, left: &str, right: &str) -> Value {
        Value::Error(ValueError::init_with(
            String::from("OperationError"),
            Some(format!("Binary operator `{op}` is not defined for types <{left}> and <{right}>")),
        ))
    }

    pub fn get_kind(&self) -> ApicaTypeBytecode {
        match self { 
            Value::Null(_) => ApicaTypeBytecode::Null,
            Value::Pointer(_) => ApicaTypeBytecode::Null,
            
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
            Value::Pointer(_) => {},
            
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
        match self {
            Value::Null(null) => null.is_null(),
            Value::Pointer(_) => false,
            
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
        match self {
            Value::Null(null) => null.get_type_representation(),
            Value::Pointer(pointer) => pointer.get_type_representation(),
            
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
    
    pub fn increment(&mut self) -> Option<Value> {
        match self {
            
            _ => None,
        }
    }

    pub fn convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        if let Some(converted) = self.auto_convert(to) {
            return Some(converted);
        }

        match self {
            Value::Null(null) => null.convert(to),
            Value::Pointer(_) => None,

            Value::I8(i8) => i8.convert(to),
            Value::I16(i16) => i16.convert(to),
            Value::I32(i32) => i32.convert(to),
            Value::I64(i64) => i64.convert(to),
            Value::U8(u8) => u8.convert(to),
            Value::U16(u16) => u16.convert(to),
            Value::U32(u32) => u32.convert(to),
            Value::U64(u64) => u64.convert(to),

            Value::F32(f32) => f32.convert(to),
            Value::F64(f64) => f64.convert(to),
            Value::Bool(bool) => bool.convert(to),

            Value::Char(char) => char.convert(to),
            Value::String(string) => string.convert(to),
            
            Value::Error(error) => error.convert(to),
            Value::Type(t) => t.convert(to),
        }
    }

    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Option<Value> {
        match self {
            Value::Null(null) => Some(null.auto_convert(to)),
            Value::Pointer(_) => None,
            
            Value::I8(i8) => i8.auto_convert(to),
            Value::I16(i16) => i16.auto_convert(to),
            Value::I32(i32) => i32.auto_convert(to),
            Value::I64(i64) => i64.auto_convert(to),
            Value::U8(u8) => u8.auto_convert(to),
            Value::U16(u16) => u16.auto_convert(to),
            Value::U32(u32) => u32.auto_convert(to),
            Value::U64(u64) => u64.auto_convert(to),

            Value::F32(f32) => f32.auto_convert(to),
            Value::F64(f64) => f64.auto_convert(to),
            Value::Bool(bool) => bool.auto_convert(to),

            Value::Char(char) => char.auto_convert(to),
            Value::String(string) => string.auto_convert(to),
            
            Value::Error(error) => error.auto_convert(to),
            Value::Type(t) => t.auto_convert(to),
        }
    }
}