pub struct ValueI8 {
    value: Option<i8>,
}

impl ValueI8 {
    pub fn init_empty() -> ValueI8 {
        return ValueI8 { value: None };
    }

    pub fn init_with(value: i8) -> ValueI8 {
        return ValueI8 { value: Some(value) };
    }
    
    pub fn show(&self, end: char) {
        if let Some(val) = self.value {
            print!("{val}{end}")
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        return self.value.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "i8";
    }
    
    pub fn get_value(&self) -> Option<i8> {
        return self.value;
    }
}