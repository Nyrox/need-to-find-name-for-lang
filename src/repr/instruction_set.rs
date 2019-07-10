use repr;

#[derive(Debug)]
pub enum Instruction {
	AddI32,
    SubI32,
    MulI32,
    DivI32,
    LessI32,

    AddF32,
    SubF32,
    MulF32,
    DivF32,
    LessF32,

    // casts
    CastI32F32,
    CastF32I32,

    Call(i16),
    CondJmp(i16),

    VarLookup(i16),
    VarAssign(i16),
	ConstI32(i16),
    ConstF32(i16),
    PopStack,
    Print(repr::Type),

    Return,
    PushVoid,
}
