use crate::element::Element;

pub struct ValueElementPointer {
    element: Element,
}

impl ValueElementPointer {
    pub fn init(element: Element) -> ValueElementPointer {
        return ValueElementPointer { element };
    }

    pub fn show(&self, end: char) {
        print!("elt-pointer<");
        self.element.get_value().show('\0');
        print!(">{end}");
    }

    pub fn is_null(&self) -> bool {
        return self.element.get_value().is_null();
    }

    pub fn get_type_representation(&self) -> &str {
        return self.element.get_value().get_type_representation();
    }
    
    pub fn get_value(&self) -> &Element {
        return &self.element;
    }
}