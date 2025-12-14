use crate::bytecodes::ApicaTypeBytecode;

pub struct ValueType {
    kind: Option<ApicaTypeBytecode>,
    contained: Vec<ValueType>,
}

fn get_kind_repr(kind: &ApicaTypeBytecode) -> &'static str {
    return match kind {
        ApicaTypeBytecode::Null => "null",
        ApicaTypeBytecode::Any => "any",
        ApicaTypeBytecode::I8 => "i8",
        ApicaTypeBytecode::I16 => "i16",
        ApicaTypeBytecode::I32 => "i32",
        ApicaTypeBytecode::I64 => "i64",
        ApicaTypeBytecode::U8 => "u8",
        ApicaTypeBytecode::U16 => "u16",
        ApicaTypeBytecode::U32 => "u32",
        ApicaTypeBytecode::U64 => "u64",
        ApicaTypeBytecode::F32 => "f32",
        ApicaTypeBytecode::F64 => "f64",
        ApicaTypeBytecode::Bool => "bool",
        ApicaTypeBytecode::Char => "char",
        ApicaTypeBytecode::String => "string",
        ApicaTypeBytecode::Error => "error",
        ApicaTypeBytecode::Type => "type",
    }
}

impl ValueType {
    pub fn init_empty() -> ValueType {
        return ValueType { kind: None, contained: vec![] };
    }

    pub fn init_with(kind: ApicaTypeBytecode, contained: Vec<ValueType>) -> ValueType {
        return ValueType { kind: Some(kind), contained };
    }
    
    pub fn show(&self, end: char) {
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
    
    pub fn is_null(&self) -> bool {
        return self.kind.is_none();
    }
    
    pub fn get_type_representation(&self) -> &str {
        return "type";
    }

    pub fn get_kind(&self) -> &Option<ApicaTypeBytecode> {
        return &self.kind;
    }

    pub fn get_contained(&self) -> &Vec<ValueType> {
        return &self.contained;
    }
}