use num_enum::TryFromPrimitive;

#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone, TryFromPrimitive)]
pub enum ApicaBytecode {
    EndOfFile =             0x0000_0000,
    EndOfBlock =            0x0000_0001,
    Entrypoint =            0x0000_0002,
    BuiltinFuncCall =       0x0000_0003,
    Literal =               0x0000_0004,
    Global =                0x0000_0005,
    Compound =              0x0000_0006,
    VarConstCall =          0x0000_0007,
    VarDecl =               0x0000_0008,
    ConstDecl =             0x0000_0009,
    Add =                   0x0000_000A,
    Subtract =              0x0000_000B,
    Assign =                0x0000_000C,
    Increment =             0x0000_000D,
    Decrement =             0x0000_000E,
    LessThan =              0x0000_000F,
    Equals =                0x0000_0010,
    Not =                   0x0000_0011,
    Break =                 0x0000_0100,
    Continue =              0x0000_0101,
    BlankReturn =           0x0000_0102,
    FilledReturn =          0x0000_0103,
    QuestionOperation =     0x0000_0104,
    If =                    0x0000_0105,
    IfElse =                0x0000_0106,
    While =                 0x0000_0107,
}

#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone, TryFromPrimitive)]
pub enum ApicaTypeBytecode {
    Null =      0x0000_0000,
    Any =       0x0000_0001,
    I8 =        0x0000_0002,
    I16 =       0x0000_0003,
    I32 =       0x0000_0004,
    I64 =       0x0000_0005,
    U8 =        0x0000_0006,
    U16 =       0x0000_0007,
    U32 =       0x0000_0008,
    U64 =       0x0000_0009,
    F32 =       0x0000_000A,
    F64 =       0x0000_000B,
    Bool =      0x0000_000C,
    Char =      0x0000_000D,
    String =    0x0000_000E,
    Error =     0x0000_000F,
    Type =      0x0000_0010,
}

#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone, TryFromPrimitive)]
pub enum ApicaEntrypointBytecode {
    Init =      0x0000_0000,
    Update =    0x0000_0001,
    Quit =      0x0000_0002,
}

#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone, TryFromPrimitive)]
pub enum ApicaBuiltinFunctionBytecode {
    Quit =              0x0000_0000,
    LogInfo =           0x0000_0001,
    LognInfo =          0x0000_0002,
    LogSuccess =        0x0000_0003,
    LognSuccess =       0x0000_0004,
    LogWarning =        0x0000_0005,
    LognWarning =       0x0000_0006,
    LogError =          0x0000_0007,
    LognError =         0x0000_0008,
    LoadApp =           0x0000_0009,
    SetTitle =          0x0000_000A,
    SetResizable =      0x0000_000B,
    IsKeyReleased =     0x0000_000C,
    IsKeyJustPressed =  0x0000_000D,
    IsKeyPressed =      0x0000_000E,
}

#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone, TryFromPrimitive)]
pub enum ApicaSpecificationBytecode {
    EndOfSpecification =    0x0000_0000,
    Title =                 0x0000_0001,
    Id =                    0x0000_0002,
    LoggerActivation =      0x0000_0003,
    WindowWidth =           0x0000_0004,
    WindowHeight =          0x0000_0005,
}

#[cfg(test)]
mod tests {
    use num_enum::TryFromPrimitive;
    use crate::bytecodes::{ApicaBuiltinFunctionBytecode, ApicaBytecode, ApicaEntrypointBytecode, ApicaSpecificationBytecode, ApicaTypeBytecode};

    #[test]
    fn test_bytecode_from() {
        let mut bytecode = ApicaBytecode::try_from_primitive(0xffffffff);
        assert!(bytecode.is_err());

        bytecode = ApicaBytecode::try_from_primitive(0);
        assert!(bytecode.is_ok());
        assert_eq!(bytecode.unwrap(), ApicaBytecode::EndOfFile);
    }

    #[test]
    fn test_type_bytecode_from() {
        let mut type_bytecode = ApicaTypeBytecode::try_from_primitive(0xffffffff);
        assert!(type_bytecode.is_err());

        type_bytecode = ApicaTypeBytecode::try_from_primitive(0);
        assert!(type_bytecode.is_ok());
        assert_eq!(type_bytecode.unwrap(), ApicaTypeBytecode::Null);
    }

    #[test]
    fn test_entry_bytecode_from() {
        let mut entry_bytecode = ApicaEntrypointBytecode::try_from_primitive(0xffffffff);
        assert!(entry_bytecode.is_err());

        entry_bytecode = ApicaEntrypointBytecode::try_from_primitive(0);
        assert!(entry_bytecode.is_ok());
        assert_eq!(entry_bytecode.unwrap(), ApicaEntrypointBytecode::Init);
    }

    #[test]
    fn test_builtin_func_bytecode_from() {
        let mut builtin_func_bytecode = ApicaBuiltinFunctionBytecode::try_from_primitive(0xffffffff);
        assert!(builtin_func_bytecode.is_err());

        builtin_func_bytecode = ApicaBuiltinFunctionBytecode::try_from_primitive(0);
        assert!(builtin_func_bytecode.is_ok());
        assert_eq!(builtin_func_bytecode.unwrap(), ApicaBuiltinFunctionBytecode::Quit);
    }

    #[test]
    fn test_specification_bytecode_from() {
        let mut specification_bytecode = ApicaSpecificationBytecode::try_from_primitive(0xffffffff);
        assert!(specification_bytecode.is_err());

        specification_bytecode = ApicaSpecificationBytecode::try_from_primitive(0);
        assert!(specification_bytecode.is_ok());
        assert_eq!(specification_bytecode.unwrap(), ApicaSpecificationBytecode::EndOfSpecification);
    }
}