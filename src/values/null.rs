use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::ValueType;
use crate::values::any::ValueAny;
use crate::values::bool::ValueBool;
use crate::values::char::ValueChar;
use crate::values::error::ValueError;
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

pub struct ValueNull {

}

impl ValueNull {
    pub fn init() -> ValueNull {
        return ValueNull{};
    }
    
    pub fn show(&self, end: char) {
        print!("null{end}");
    }
    
    pub fn is_null(&self) -> bool {
        return true;
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "null";
    }

    pub fn convert(&'_ self, _: ApicaTypeBytecode) -> Option<Value<'_>> {
        return None; // null is AUTOMATICALLY converted
    }

    pub fn auto_convert(&'_ self, to: ApicaTypeBytecode) -> Value<'_> {
        return match to {
            ApicaTypeBytecode::Null => Value::Null(ValueNull::init()),
            ApicaTypeBytecode::Any => Value::Any(Box::new(ValueAny::init_empty())),
            ApicaTypeBytecode::I8 => Value::I8(ValueI8::init_empty()),
            ApicaTypeBytecode::I16 => Value::I16(ValueI16::init_empty()),
            ApicaTypeBytecode::I32 => Value::I32(ValueI32::init_empty()),
            ApicaTypeBytecode::I64 => Value::I64(ValueI64::init_empty()),
            ApicaTypeBytecode::U8 => Value::U8(ValueU8::init_empty()),
            ApicaTypeBytecode::U16 => Value::U16(ValueU16::init_empty()),
            ApicaTypeBytecode::U32 => Value::U32(ValueU32::init_empty()),
            ApicaTypeBytecode::U64 => Value::U64(ValueU64::init_empty()),
            ApicaTypeBytecode::F32 => Value::F32(ValueF32::init_empty()),
            ApicaTypeBytecode::F64 => Value::F64(ValueF64::init_empty()),
            ApicaTypeBytecode::Bool => Value::Bool(ValueBool::init_empty()),
            ApicaTypeBytecode::Char => Value::Char(ValueChar::init_empty()),
            ApicaTypeBytecode::String => Value::String(ValueString::init_empty()),
            ApicaTypeBytecode::Error => Value::Error(ValueError::init_empty()),
            ApicaTypeBytecode::Type => Value::Type(ValueType::init_empty()),
        }
    }
}