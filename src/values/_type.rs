use crate::values::value::{Value, ValueKind};

pub struct ValueType {
    kind: Option<ValueKind>,
    contained: Vec<ValueType>,
}

fn get_kind_repr(kind: &ValueKind) -> &'static str {
    return match kind {
        ValueKind::Null => "null",
        ValueKind::I8 => "i8",
        ValueKind::I16 => "i16",
        ValueKind::I32 => "i32",
        ValueKind::I64 => "i64",
        ValueKind::U8 => "u8",
        ValueKind::U16 => "u16",
        ValueKind::U32 => "u32",
        ValueKind::U64 => "u64",
        ValueKind::F32 => "f32",
        ValueKind::F64 => "f64",
        ValueKind::Bool => "bool",
        ValueKind::Char => "char",
        ValueKind::String => "string",
        ValueKind::Error => "error",
        ValueKind::Type => "type",

        _ => "",
    }
}

impl Value for ValueType {
    fn get_kind(&self) -> ValueKind {
        return ValueKind::Type;
    }

    fn show(&self, end: char) {
        if let Some(kind) = &self.kind {
            let repr = get_kind_repr(kind);
            print!("{repr}");
            if !self.contained.is_empty() {
                print!("<");
                for i in 0..self.contained.len() {
                    let c = &self.contained[i];
                    c.show('\0');
                    if i != self.contained.len() - 1 {
                        print!(", ");
                    }
                }

                print!(">");
            }
        } else {
            print!("null{end}");
        }
    }

    fn is_null(&self) -> bool {
        return self.kind.is_none();
    }

    fn get_type_representation(&self) -> &str {
        return "type";
    }
}

impl ValueType {
    pub fn init_empty() -> ValueType {
        return ValueType { kind: None, contained: vec![] };
    }

    pub fn init_with(kind: ValueKind, contained: Vec<ValueType>) -> ValueType {
        return ValueType { kind: Some(kind), contained };
    }

    pub fn get_kind(&self) -> &Option<ValueKind> {
        return &self.kind;
    }

    pub fn get_contained(&self) -> &Vec<ValueType> {
        return &self.contained;
    }
}