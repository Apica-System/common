use crate::bytecodes::ApicaTypeBytecode;
use crate::context::Context;
use crate::values::value::Value;

pub struct ValueAny {
    internal: Option<Value>,
}

impl ValueAny {
    pub fn init_empty() -> ValueAny {
        return ValueAny { internal: None };
    }
    
    pub fn init_with(value: Value) -> ValueAny {
        return ValueAny { internal: Some(value) };
    }
    
    pub fn get_kind(&self, context: &Context) -> ApicaTypeBytecode {
        return if let Some(val) = self.get_value() {
            val.get_kind(context)
        } else {
            ApicaTypeBytecode::Null
        }
    }
    
    pub fn show(&self, end: char, context: &Context) {
        print!("any<");
        if let Some(val) = &self.internal {
            val.show('\0', context);
        } else {
            print!("null");
        }

        print!(">{end}");
    }
    
    pub fn is_null(&self) -> bool {
        return self.internal.is_none();
    }
    
    pub fn get_type_representation<'a>(&'a self, context: &'a Context) -> &'a str {
        return if let Some(val) = &self.internal {
            val.get_type_representation(context)
        } else {
            "any<null>"
        }
    }
    
    pub fn get_value(&self) -> &Option<Value> {
        return &self.internal;
    }

    pub fn convert(&self, to: ApicaTypeBytecode, context: &Context) -> Option<Value> {
        if let Some(val) = &self.internal {
            return val.convert(to, context);
        }

        return None;
    }

    pub fn auto_convert(&self, to: ApicaTypeBytecode, context: &Context) -> Option<Value> {
        if let Some(val) = &self.internal {
            return val.auto_convert(to, context);
        }
        
        return None;
    }
}