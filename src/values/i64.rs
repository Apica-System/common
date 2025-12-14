pub struct ValueI64 {
    value: Option<i64>,
}

impl ValueI64 {
    pub fn init_empty() -> ValueI64 {
        return ValueI64 { value: None };
    }
    
    pub fn init_with(value: i64) -> ValueI64 {
        return ValueI64 { value: Some(value) };
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
        return "i64";
    }
    
    pub fn get_value(&self) -> Option<i64> {
        return self.value;
    }
}