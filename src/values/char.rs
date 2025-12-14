pub struct ValueChar {
    value: Option<char>,
}

impl ValueChar {
    pub fn init_empty() -> ValueChar {
        return ValueChar { value: None };
    }

    pub fn init_with(value: char) -> ValueChar {
        return ValueChar { value: Some(value) };
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
        return "char";
    }
    
    pub fn get_value(&self) -> Option<char> {
        return self.value;
    }
}