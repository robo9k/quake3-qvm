// These should match the names in ioquake3
#[allow(non_camel_case_types)]
/// Operation code for a QVM instruction.
///
/// See ioquake3's `opcode_t` in `qcommon/vm_local.h`
pub enum Opcode {
    UNDEF,

    IGNORE,

    BREAK,

    ENTER,
    LEAVE,
    CALL,
    PUSH,
    POP,

    CONST,
    LOCAL,

    JUMP,

    EQ,
    NE,

    LTI,
    LEI,
    GTI,
    GEI,

    LTU,
    LEU,
    GTU,
    GEU,

    EQF,
    NEF,

    LTF,
    LEF,
    GTF,
    GEF,

    LOAD1,
    LOAD2,
    LOAD4,
    STORE1,
    STORE2,
    STORE4,
    ARG,

    BLOCK_COPY,

    SEX8,
    SEX16,

    NEGI,
    ADD,
    SUB,
    DIVI,
    DIVU,
    MODI,
    MODU,
    MULI,
    MULU,

    BAND,
    BOR,
    BXOR,
    BCOM,

    LSH,
    RSHI,
    RSHU,

    NEGF,
    ADDF,
    SUBF,
    DIVF,
    MULF,

    CVIF,
    CVFI,
}
