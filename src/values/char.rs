use crate::values::value::{Value, ValueKind};

pub struct ValueChar {
    value: Option<char>,
}

impl Value for ValueChar {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::Char;
    }

    fn show(&self, end: char) {
        if let Some(val) = self.value {
            print!("{val}{end}");
        } else {
            print!("null{end}");
        }
    }

    fn is_null(&self) -> bool {
        return self.value.is_none();
    }

    fn get_type_representation(&self) -> &str {
        return "char";
    }
}

impl ValueChar {
    pub fn init_empty() -> ValueChar {
        return ValueChar { value: None };
    }

    pub fn init_with(value: char) -> ValueChar {
        return ValueChar { value: Some(value) };
    }

    pub fn get_value(&self) -> Option<char> {
        return self.value;
    }
}