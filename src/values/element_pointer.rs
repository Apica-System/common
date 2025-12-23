use crate::bytecodes::ApicaTypeBytecode;
use crate::element::Element;
use crate::values::value::Value;

pub struct ValueElementPointer<'a> {
    element: &'a mut Element<'a>,
}

impl<'a> ValueElementPointer<'a> {
    pub fn init(element: &'a mut Element<'a>) -> ValueElementPointer<'a> {
        return ValueElementPointer { element };
    }

    pub fn show(&'a self, end: char) {
        print!("elt-pointer<");
        self.element.get_value().show('\0');
        print!(">{end}");
    }

    pub fn is_null(&'a self) -> bool {
        return self.element.get_value().is_null();
    }

    pub fn get_type_representation(&'a self) -> &'a str {
        return self.element.get_value().get_type_representation();
    }
    
    pub fn get_value(&'a self) -> &'a Element<'a> {
        return &self.element;
    }

    pub fn convert(&'a self, to: ApicaTypeBytecode) -> Option<Value<'a>> {
        return self.element.get_value().convert(to);
    }

    pub fn auto_convert(&'a self, to: ApicaTypeBytecode) -> Option<Value<'a>> {
        return self.element.get_value().auto_convert(to);
    }
}