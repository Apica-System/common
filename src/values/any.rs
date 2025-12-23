use crate::bytecodes::ApicaTypeBytecode;
use crate::values::value::Value;

pub struct ValueAny<'a> {
    internal: Option<Value<'a>>,
}

impl<'a> ValueAny<'a> {
    pub fn init_empty() -> ValueAny<'a> {
        return ValueAny { internal: None };
    }
    
    pub fn init_with(value: Value) -> ValueAny {
        return ValueAny { internal: Some(value) };
    }
    
    pub fn show(&'a self, end: char) {
        print!("any<");
        if let Some(val) = &self.internal {
            val.show('\0');
        } else {
            print!("null");
        }

        print!(">{end}");
    }
    
    pub fn is_null(&self) -> bool {
        return self.internal.is_none();
    }
    
    pub fn get_type_representation(&'a self) -> &'a str {
        return if let Some(val) = &self.internal {
            val.get_type_representation()
        } else {
            "any<null>"
        }
    }
    
    pub fn get_value(&self) -> &Option<Value<'a>> {
        return &self.internal;
    }

    pub fn convert(&'a self, to: ApicaTypeBytecode) -> Option<Value<'a>> {
        if let Some(val) = &self.internal {
            return val.convert(to);
        }

        return None;
    }

    pub fn auto_convert(&'a self, to: ApicaTypeBytecode) -> Option<Value<'a>> {
        if let Some(val) = &self.internal {
            return val.auto_convert(to);
        }
        
        return None;
    }
}