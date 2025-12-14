#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone)]
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
    Substract =             0x0000_000B,
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
#[derive(PartialEq, Debug, Copy, Clone)]
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
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ApicaEntrypointBytecode {
    Init =      0x0000_0000,
    Update =    0x0000_0001,
    Quit =      0x0000_0002,
}

#[repr(u64)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ApicaBuiltinFunctionBytecode {
    Quit =          0x00000000,
    LogInfo =       0x00000001,
    LognInfo =      0x00000002,
    LogSuccess =    0x00000003,
    LognSuccess =   0x00000004,
    LogWarning =    0x00000005,
    LognWarning =   0x00000006,
    LogError =      0x00000007,
    LognError =     0x00000008,
    LoadApp =       0x00000009,
    SetTitle =      0x0000000A,
    SetResizable =  0x0000000B,
}