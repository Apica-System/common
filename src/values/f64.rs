pub struct ValueF64 {
    value: Option<f64>,
}

impl ValueF64 {
    pub fn init_empty() -> ValueF64 {
        return ValueF64 { value: None };
    }

    pub fn init_with(value: f64) -> ValueF64 {
        return ValueF64 { value: Some(value) };
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
        return "f64";
    }

    pub fn get_value(&self) -> Option<f64> {
        return self.value;
    }
}