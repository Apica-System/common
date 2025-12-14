pub struct ValueU64 {
    value: Option<u64>,
}

impl ValueU64 {
    pub fn init_empty() -> ValueU64 {
        ValueU64 { value: None }
    }
    
    pub fn init_with(value: u64) -> ValueU64 {
        ValueU64 { value: Some(value) }
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
        return "u64";
    }
    
    pub fn get_value(&self) -> Option<u64> {
        return self.value
    }
}