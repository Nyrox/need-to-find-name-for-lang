use repr;

#[derive(Debug)]
pub enum Instruction {
	ADD_I32,
    SUB_I32,
    MUL_I32,
    DIV_I32,

    ADD_F32,
    SUB_F32,
    MUL_F32,
    DIV_F32,

    // casts
    CAST_I32_F32,
    CAST_F32_I32,

    CALL(i16),
    VAR_LOOKUP(i16),
    VAR_ASSIGN(i16),
	CONST_I32(i16),
    CONST_F32(i16),
    POP_STACK,
    PRINT(repr::Type),

    RETURN,
}