use crate::values::value::{Value, ValueKind};

pub struct ValueNull {

}

impl Value for ValueNull {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::Null;
    }

    fn show(&self, end: char) {
        print!("null{end}");
    }

    fn is_null(&self) -> bool {
        return true;
    }

    fn get_type_representation(&self) -> &str {
        return "null";
    }
}

impl ValueNull {
    pub fn init() -> ValueNull {
        return ValueNull{};
    }
}