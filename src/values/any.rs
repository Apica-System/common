use crate::values::value::{Value, ValueKind};

pub struct ValueAny {
    internal: Option<Box<dyn Value>>,
}

impl Value for ValueAny {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::Any
    }

    fn show(&self, end: char) {
        print!("any<");
        if let Some(val) = &self.internal {
            val.show('\0');
        } else {
            print!("null");
        }

        print!(">{end}");
    }

    fn is_null(&self) -> bool {
        return self.internal.is_none();
    }

    fn get_type_representation(&self) -> &str {
        if let Some(val) = &self.internal {
            return val.get_type_representation();
        } else {
            return "any<null>";
        }
    }
}

impl ValueAny {
    pub fn init_empty() -> ValueAny {
        return ValueAny { internal: None };
    }
    
    pub fn init_with(value: Box<dyn Value>) -> ValueAny {
        return ValueAny { internal: Some(value) };
    }
    
    pub fn get_value(&self) -> &Option<Box<dyn Value>> {
        return &self.internal;
    }
}