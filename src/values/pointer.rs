pub struct ValuePointer {
    pointer: String,
}

impl ValuePointer {
    pub fn init_with(pointer: String) -> ValuePointer {
        ValuePointer { pointer }
    }

    pub fn get_type_representation(&self) -> &str {
        "elt-pointer"
    }

    pub fn get_pointer(&self) -> &str {
        &self.pointer
    }
}