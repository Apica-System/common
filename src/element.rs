use bitflags::bitflags;
use crate::values::value::Value;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct ElementModifier: u8 {
        const None =    0b00000000;
        const Error =   0b00000001;
        const Const =   0b00000010;
    }
}

pub struct Element {
    modifier: ElementModifier,
    value: Box<dyn Value>,
}

impl Element {
    pub fn init(modifier: ElementModifier, value: Box<dyn Value>) -> Element {
        return Element{modifier, value};
    }

    pub fn get_value(&self) -> &Box<dyn Value> {
        return &self.value;
    }

    pub fn get_modifier(&self) -> ElementModifier {
        return self.modifier;
    }

    pub fn add_modifier(&mut self, modifier: ElementModifier) {
        self.modifier |= modifier;
    }
}