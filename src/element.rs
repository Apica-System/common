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

    pub fn check_convert(self, to: ApicaTypeBytecode) -> Element {
        if self.is_error_or_controller() {
            return self;
        }

        if self.get_value().get_kind() == to {
            self
        } else {
            self.auto_convert(to)
        }
    }
    
    pub fn add(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::create_error(Value::null_operation_error("+", false));
        }
        
        if let Some(added) = self.value.add(other.get_value()) {
            Element::init(ElementModifier::None, added)
        } else {
            Element::create_error(Value::binary_operation_error("+", self.value.get_type_representation(), other.value.get_type_representation()))
        }
    }

    pub fn increment(&mut self) -> Element {
        if self.value.is_null() {
            return Element::create_error(Value::null_operation_error("right ++", true));
        }
        
        if let Some(incremented) = self.value.increment() {
            Element::init(ElementModifier::None, incremented)
        } else {
            Element::create_error(Value::unary_operation_error("right ++", self.value.get_type_representation()))
        }
    }

    pub fn decrement(&mut self) -> Element {
        if self.value.is_null() {
            return Element::create_error(Value::null_operation_error("right --", true));
        }
        
        if let Some(decremented) = self.value.decrement() {
            Element::init(ElementModifier::None, decremented)
        } else {
            Element::create_error(Value::unary_operation_error("right --", self.value.get_type_representation()))
        }
    }

    pub fn not(&mut self) -> Element {
        Element::init(ElementModifier::None, self.value.not())
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