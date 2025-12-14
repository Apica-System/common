pub struct ValueBool {
    value: Option<bool>,
}

impl ValueBool {
    pub fn init_empty() -> ValueBool {
        return ValueBool { value: None };
    }
    
    pub fn init_with(value: bool) -> ValueBool {
        return ValueBool { value: Some(value) };
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
        return "bool";
    }
    
    pub fn get_value(&self) -> Option<bool> {
        return self.value;
    }
}