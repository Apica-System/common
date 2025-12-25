use bitflags::bitflags;
use crate::bytecodes::ApicaTypeBytecode;
use crate::values::_type::get_kind_repr;
use crate::values::null::ValueNull;
use crate::values::value::Value;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct ElementModifier: u8 {
        const None =        0b0000_0000;
        const Error =       0b0000_0001;
        const Const =       0b0000_0010;
        const Controller =  0b0000_0100;
        const Any =         0b0000_1000;
    }
}

pub struct Element {
    modifier: ElementModifier,
    value: Value,
}

impl Element {
    pub fn init(modifier: ElementModifier, value: Value) -> Element {
        Element{modifier, value}
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }
    
    pub fn get_value_mut(&mut self) -> &mut Value {
        &mut self.value
    }

    pub fn get_modifier(&self) -> ElementModifier {
        self.modifier
    }

    pub fn add_modifier(&mut self, modifier: ElementModifier) {
        self.modifier |= modifier;
    }

    pub fn create_null() -> Element {
        Element{ modifier: ElementModifier::None, value: Value::Null(ValueNull::init()) }
    }

    pub fn create_error(value: Value) -> Element {
        Element{ modifier: ElementModifier::Error, value }
    }

    pub fn is_error_or_controller(&self) -> bool {
        self.modifier.contains(ElementModifier::Error) || self.modifier.contains(ElementModifier::Controller)
    }

    pub fn convert(&self, to: ApicaTypeBytecode) -> Element {
        if let Some(converted) = self.value.convert(to) {
            Element::init(ElementModifier::None, converted)
        } else {
            Element::create_error(Value::binary_operation_error("as", self.value.get_type_representation(), get_kind_repr(&to)))
        }
    }

    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Element {
        if let Some(auto_converted) = self.value.auto_convert(to) {
            Element::init(ElementModifier::None, auto_converted)
        } else {
            Element::create_error(Value::binary_operation_error("auto-as", self.value.get_type_representation(), get_kind_repr(&to)))
        }
    }
}