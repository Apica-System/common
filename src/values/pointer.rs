use crate::bytecodes::ApicaTypeBytecode;
use crate::context::Context;
use crate::values::value::Value;

pub struct ValuePointer {
    pointer: String,
}

impl ValuePointer {
    pub fn init(pointer: String) -> ValuePointer {
        return ValuePointer { pointer };
    }

    pub fn get_kind(&self, context: &Context) -> ApicaTypeBytecode {
        return if let Some(element) = context.get_element(self.pointer.as_str(), false) {
            element.get_value().get_kind(context)
        } else {
            ApicaTypeBytecode::Null
        }
    }

    pub fn show(&self, end: char, context: &Context) {
        print!("elt-pointer<");
        if let Some(element) = context.get_element(self.pointer.as_str(), false) {
            element.get_value().show('\0', context);
        } else {
            print!("???");
        }

        print!(">{end}");
    }

    pub fn is_null(&self, context: &Context) -> bool {
        return if let Some(element) = context.get_element(self.pointer.as_str(), false) {
            element.get_value().is_null(context)
        } else {
            true
        }
    }

    pub fn get_type_representation<'a>(&self, context: &'a Context) -> &'a str {
        return if let Some(element) = context.get_element(self.pointer.as_str(), false) {
            element.get_value().get_type_representation(context)
        } else {
            "null"
        }
    }
    
    pub fn get_pointer(&self) -> &String {
        return &self.pointer;
    }

    pub fn convert(&self, to: ApicaTypeBytecode, context: &Context) -> Option<Value> {
        return if let Some(element) = context.get_element(self.pointer.as_str(), false) {
            element.get_value().convert(to, context)
        } else {
            None
        }
    }

    pub fn auto_convert(&self, to: ApicaTypeBytecode, context: &Context) -> Option<Value> {
        return if let Some(element) = context.get_element(self.pointer.as_str(), false) {
            element.get_value().auto_convert(to, context)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bytecodes::ApicaTypeBytecode;
    use crate::context::Context;
    use crate::element::{Element, ElementModifier};
    use crate::values::bool::ValueBool;
    use crate::values::pointer::ValuePointer;
    use crate::values::value::Value;

    #[test]
    pub fn use_pointer() {
        let mut context = Context::init();
        let var_name = String::from("variable");

        assert_eq!(true, context.set_element(
            var_name.clone(),
            Element::init(ElementModifier::None, Value::Bool(ValueBool::init_with(true))),
            false,
        ));

        let pointer = Element::init(ElementModifier::None, Value::Pointer(ValuePointer::init(var_name.clone())));
        assert_eq!("bool", pointer.get_value().get_type_representation(&context));

        let integer = pointer.convert(ApicaTypeBytecode::I8, &context);
        if let Value::I8(i8) = integer.get_value() {
            assert_eq!(1, i8.get_value().unwrap());
        } else {
            panic!("Converted integer should be a i8");
        }
    }
}