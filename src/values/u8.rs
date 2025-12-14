pub struct ValueU8 {
    value: Option<u8>,
}

impl ValueU8 {
    pub fn init_empty() -> ValueU8 {
        return ValueU8 { value: None };
    }

    pub fn init_with(value: u8) -> ValueU8 {
        return ValueU8 { value: Some(value) };
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
        return "u8";
    }
    
    pub fn get_value(&self) -> Option<u8> {
        return self.value;
    }
}