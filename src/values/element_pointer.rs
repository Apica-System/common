use crate::element::Element;
use crate::values::value::{Value, ValueKind};

pub struct ValueElementPointer {
    element: Element,
}

impl Value for ValueElementPointer {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::ElementPointer;
    }

    fn show(&self, end: char) {
        print!("elt-pointer<");
        self.element.get_value().show('\0');
        print!(">{end}");
    }

    fn is_null(&self) -> bool {
        return self.element.get_value().is_null();
    }

    fn get_type_representation(&self) -> &str {
        return self.element.get_value().get_type_representation();
    }
}

impl ValueElementPointer {
    pub fn init(element: Element) -> ValueElementPointer {
        return ValueElementPointer { element };
    }
    
    pub fn get_value(&self) -> &Element {
        return &self.element;
    }
}