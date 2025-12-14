pub struct ValueString {
    value: Option<String>,
}

impl ValueString {
    pub fn init_empty() -> ValueString {
        return ValueString { value: None };
    }

    pub fn init_with(value: String) -> ValueString {
        return ValueString { value: Some(value) };
    }

    pub fn show(&self, end: char) {
        if let Some(val) = &self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        return self.value.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "string";
    }
    
    pub fn init_from(value: &str) -> ValueString {
        return ValueString { value: Some(String::from(value)) };
    }

    pub fn get_value(&self) -> &Option<String> {
        return &self.value;
    }
}