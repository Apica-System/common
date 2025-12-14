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
    
    pub fn show(&self, end: char) {
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
    
    pub fn get_type_representation(&self) -> &str {
        return if let Some(val) = &self.internal {
            val.get_type_representation()
        } else {
            "any<null>"
        }
    }
    
    pub fn get_value(&self) -> &Option<Value> {
        return &self.internal;
    }
}