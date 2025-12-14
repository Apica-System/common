pub struct ValueI16 {
    value: Option<i16>,
}

impl ValueI16 {
    pub fn init_empty() -> ValueI16 {
        return ValueI16 { value: None };
    }

    pub fn init_with(value: i16) -> ValueI16 {
        return ValueI16 { value: Some(value) };
    }

    pub fn show(&self, end: char) {
        if let Some(val) = self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }
    
    pub fn is_null(&self) -> bool {
        self.value.is_none()
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "i16";
    }
    
    pub fn get_value(&self) -> Option<i16> {
        return self.value;
    }
}