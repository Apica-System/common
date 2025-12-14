pub struct ValueI32 {
    value: Option<i32>,
}

impl ValueI32 {
    pub fn init_empty() -> ValueI32 {
        return ValueI32 { value: None };
    }
    
    pub fn init_with(value: i32) -> ValueI32 {
        return ValueI32 { value: Some(value) };
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
        return "i32";
    }
    
    pub fn get_value(&self) -> Option<i32> {
        return self.value;
    }
}