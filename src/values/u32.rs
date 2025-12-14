pub struct ValueU32 {
    value: Option<u32>,
}

impl ValueU32 {
    pub fn init_empty() -> ValueU32 {
        return ValueU32 { value: None };
    }
    
    pub fn init_with(value: u32) -> ValueU32 {
        return ValueU32 { value: Some(value) };
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
        return "u32";
    }
    
    pub fn get_value(&self) -> Option<u32> {
        return self.value;
    }
}