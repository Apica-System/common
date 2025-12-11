use crate::values::value::{Value, ValueKind};

pub struct ValueError {
    name: Option<String>,
    details: Option<String>,
}

impl Value for ValueError {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::Error;
    }

    fn show(&self, end: char) {
        if let Some(name) = &self.name {
            print!("{name}");
            if let Some(details) = &self.details {
                print!(": {details}");
            }
            print!("{end}");
        } else {
            print!("null{end}");
        }
    }

    fn is_null(&self) -> bool {
        return self.name.is_none();
    }

    fn get_type_representation(&self) -> &str {
        return "error";
    }
}

impl ValueError {
    pub fn init_empty() -> ValueError {
        return ValueError { name: None, details: None };
    }

    pub fn init_with(name: String, details: Option<String>) -> ValueError {
        return ValueError { name: Some(name), details };
    }

    pub fn get_name(&self) -> &Option<String> {
        return &self.name;
    }

    pub fn get_details(&self) -> &Option<String> {
        return &self.details;
    }
}