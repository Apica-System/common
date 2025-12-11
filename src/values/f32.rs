use crate::values::value::{Value, ValueKind};

pub struct ValueF32 {
    value: Option<f32>,
}

impl Value for ValueF32 {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::F32;
    }

    fn show(&self, end: char) {
        if let Some(val) = self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }

    fn is_null(&self) -> bool {
        return self.value.is_none();
    }

    fn get_type_representation(&self) -> &str {
        return "f32";
    }
}

impl ValueF32 {
    pub fn init_empty() -> ValueF32 {
        return ValueF32 { value: None };
    }
    
    pub fn init_with(value: f32) -> ValueF32 {
        return ValueF32 { value: Some(value) };
    }
    
    pub fn get_value(&self) -> Option<f32> {
        return self.value;
    }
}