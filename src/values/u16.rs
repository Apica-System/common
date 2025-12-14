pub struct ValueU16 {
    value: Option<u16>,
}

impl ValueU16 {
    pub fn init_empty() -> ValueU16 {
        return ValueU16 { value: None };
    }
    
    pub fn init_with(value: u16) -> ValueU16 {
        return ValueU16 { value: Some(value) };
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
        return "u16";
    }
    
    pub fn get_value(&self) -> Option<u16> {
        return self.value;
    }
}