use super::{Opcode, Instruction, QVM, VM_MAGIC};
use nom;
use nom::*;

type Input = u8;
type InputSlice<'a> = &'a [Input];

/// Creates a named parser for an instruction that only consists of an opcode
macro_rules! instruction {
    ($name:ident, $opcode:path, $instruction:path) => {
        named!($name<InputSlice,Instruction>,
            value!($instruction, tag!([$opcode as Input]))
        );
    }
}

/// Creates a named parser for an instruction that has an operand
macro_rules! instruction_with_operand {
    ($name:ident, $opcode:path, $operand_fn:ident, $instruction:path) => {
        named!($name<InputSlice,Instruction>,
            do_parse!(
                tag!([$opcode as Input]) >>
                operand: $operand_fn      >>
                ($instruction(operand))
            )
        );
    }
}

/// Creates a named parser for an instruction with `u32` operand
macro_rules! instruction_u32 {
    ($name:ident, $opcode:path, $instruction:path) => {
        instruction_with_operand!($name, $opcode, le_u32, $instruction);
    }
}

/// Creates a named parser for an instruction with `u8` operand
macro_rules! instruction_u8 {
    ($name:ident, $opcode:path, $instruction:path) => {
        instruction_with_operand!($name, $opcode, le_u8, $instruction);
    }
}


instruction!(op_undef, Opcode::UNDEF, Instruction::UNDEF);

instruction!(op_ignore, Opcode::IGNORE, Instruction::IGNORE);

instruction!(op_break, Opcode::BREAK, Instruction::BREAK);

instruction_u32!(op_enter, Opcode::ENTER, Instruction::ENTER);
instruction_u32!(op_leave, Opcode::LEAVE, Instruction::LEAVE);
instruction!(op_call, Opcode::CALL, Instruction::CALL);
instruction!(op_push, Opcode::PUSH, Instruction::PUSH);
instruction!(op_pop, Opcode::POP, Instruction::POP);

instruction_u32!(op_const, Opcode::CONST, Instruction::CONST);
instruction_u32!(op_local, Opcode::LOCAL, Instruction::LOCAL);

instruction!(op_jump, Opcode::JUMP, Instruction::JUMP);

instruction_u32!(op_eq, Opcode::EQ, Instruction::EQ);
instruction_u32!(op_ne, Opcode::NE, Instruction::NE);

instruction_u32!(op_lti, Opcode::LTI, Instruction::LTI);
instruction_u32!(op_lei, Opcode::LEI, Instruction::LEI);
instruction_u32!(op_gti, Opcode::GTI, Instruction::GTI);
instruction_u32!(op_gei, Opcode::GEI, Instruction::GEI);

instruction_u32!(op_ltu, Opcode::LTU, Instruction::LTU);
instruction_u32!(op_leu, Opcode::LEU, Instruction::LEU);
instruction_u32!(op_gtu, Opcode::GTU, Instruction::GTU);
instruction_u32!(op_geu, Opcode::GEU, Instruction::GEU);

instruction_u32!(op_eqf, Opcode::EQF, Instruction::EQF);
instruction_u32!(op_nef, Opcode::NEF, Instruction::NEF);

instruction_u32!(op_ltf, Opcode::LTF, Instruction::LTF);
instruction_u32!(op_lef, Opcode::LEF, Instruction::LEF);
instruction_u32!(op_gtf, Opcode::GTF, Instruction::GTF);
instruction_u32!(op_gef, Opcode::GEF, Instruction::GEF);

instruction!(op_load1, Opcode::LOAD1, Instruction::LOAD1);
instruction!(op_load2, Opcode::LOAD2, Instruction::LOAD2);
instruction!(op_load4, Opcode::LOAD4, Instruction::LOAD4);
instruction!(op_store1, Opcode::STORE1, Instruction::STORE1);
instruction!(op_store2, Opcode::STORE2, Instruction::STORE2);
instruction!(op_store4, Opcode::STORE4, Instruction::STORE4);
instruction_u8!(op_arg, Opcode::ARG, Instruction::ARG);

instruction_u32!(op_block_copy, Opcode::BLOCK_COPY, Instruction::BLOCK_COPY);

instruction!(op_sex8, Opcode::SEX8, Instruction::SEX8);
instruction!(op_sex16, Opcode::SEX16, Instruction::SEX16);

instruction!(op_negi, Opcode::NEGI, Instruction::NEGI);
instruction!(op_add, Opcode::ADD, Instruction::ADD);
instruction!(op_sub, Opcode::SUB, Instruction::SUB);
instruction!(op_divi, Opcode::DIVI, Instruction::DIVI);
instruction!(op_divu, Opcode::DIVU, Instruction::DIVU);
instruction!(op_modi, Opcode::MODI, Instruction::MODI);
instruction!(op_modu, Opcode::MODU, Instruction::MODU);
instruction!(op_muli, Opcode::MULI, Instruction::MULI);
instruction!(op_mulu, Opcode::MULU, Instruction::MULU);

instruction!(op_band, Opcode::BAND, Instruction::BAND);
instruction!(op_bor, Opcode::BOR, Instruction::BOR);
instruction!(op_bxor, Opcode::BXOR, Instruction::BXOR);
instruction!(op_bcom, Opcode::BCOM, Instruction::BCOM);

instruction!(op_lsh, Opcode::LSH, Instruction::LSH);
instruction!(op_rshi, Opcode::RSHI, Instruction::RSHI);
instruction!(op_rshu, Opcode::RSHU, Instruction::RSHU);

instruction!(op_negf, Opcode::NEGF, Instruction::NEGF);
instruction!(op_addf, Opcode::ADDF, Instruction::ADDF);
instruction!(op_subf, Opcode::SUBF, Instruction::SUBF);
instruction!(op_divf, Opcode::DIVF, Instruction::DIVF);
instruction!(op_mulf, Opcode::MULF, Instruction::MULF);

instruction!(op_cvif, Opcode::CVIF, Instruction::CVIF);
instruction!(op_cvfi, Opcode::CVFI, Instruction::CVFI);


named!(ins<InputSlice,Instruction>,
    alt!(op_undef
        | op_ignore
        | op_break
        | op_enter | op_leave | op_call | op_push | op_pop
        | op_const | op_local
        | op_jump
        | op_eq | op_ne
        | op_lti | op_lei | op_gti | op_gei
        | op_ltu | op_leu | op_gtu | op_geu
        | op_eqf | op_nef
        | op_ltf | op_lef | op_gtf | op_gef
        | op_load1 | op_load2 | op_load4 | op_store1 | op_store2 | op_store4 | op_arg
        | op_block_copy
        | op_sex8 | op_sex16
        | op_negi | op_add | op_sub | op_divi | op_divu | op_modi | op_modu | op_muli | op_mulu
        | op_band | op_bor | op_bxor | op_bcom
        | op_lsh  | op_rshi | op_rshu
        | op_negf | op_addf | op_subf | op_divf | op_mulf
        | op_cvif | op_cvfi
    )
);

macro_rules! length_size(
    ($i:expr, $s:expr, $submac:ident!( $($args:tt)* )) => (
    {
        match take!($i, $s as usize) {
            nom::IResult::Error(e)                         => nom::IResult::Error(e),
            nom::IResult::Incomplete(nom::Needed::Unknown) => nom::IResult::Incomplete(nom::Needed::Unknown),
            nom::IResult::Incomplete(nom::Needed::Size(n)) => {
                nom::IResult::Incomplete(nom::Needed::Size(
                    n + nom::InputLength::input_len(&($i)) - ($s)
                ))
            },
            nom::IResult::Done(i2, o2)  => {
                match complete!(o2, $submac!($($args)*)) {
                    nom::IResult::Error(e)      => nom::IResult::Error(e),
                    nom::IResult::Incomplete(i) => nom::IResult::Incomplete(i),
                    nom::IResult::Done(_, o3)   => nom::IResult::Done(i2, o3)
                }
            }
        }
    }
    );

  ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
    length_value!($i, $submac!($($args)*), call!($g));
  );

  ($i:expr, $f:expr, $submac:ident!( $($args:tt)* )) => (
    length_value!($i, call!($f), $submac!($($args)*));
  );

  ($i:expr, $f:expr, $g:expr) => (
    length_value!($i, call!($f), call!($g));
  );
);

const HEADER_LENGTH_V1: u32 = 32;

named!(qvm<InputSlice, QVM>,
    do_parse!(
        magic: le_u32                                   >>
        instruction_count: le_u32                       >>
        code_offset: le_u32                             >>
        code_length: le_u32                             >>
        data_offset: le_u32                             >>
        data_length: le_u32                             >>
        lit_length: le_u32                              >>
        bss_length: le_u32                              >>
        // Read padding between header and code segment
        take!(code_offset - HEADER_LENGTH_V1)           >>
        code: length_size!(
            code_length as usize,
            count!(ins, instruction_count as usize)
        )                                               >>
        // Read padding between code and data segment
        take!(data_offset - code_offset - code_length)  >>
        data: length_size!(
            data_length as usize,
            count!(le_u32, data_length as usize / 4)
        )                                               >>
        // lit segment is always aligned, no padding here
        lit: length_size!(
            lit_length as usize,
            count!(le_u8, lit_length as usize)
        )                                               >>
        eof!()                                          >>
        (
            QVM {
                code: code,
                data: data,
                lit: lit,
                bss_length: bss_length,
            }
        )
    )
);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_op_break_exact_match() {
        let data = [0x2];
        let result = op_break(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::BREAK));
    }

    #[test]
    fn test_op_break_tag_mismatch() {
        let data = [0x0];
        let result = op_break(&data);
        assert_eq!(result, IResult::Error(ErrorKind::Tag));
    }

    #[test]
    fn test_op_enter_exact_match() {
        let data = [0x3, 0x42, 0x0, 0x0, 0x0];
        let result = op_enter(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ENTER(0x42)));
    }

    #[test]
    fn test_op_arg_exact_match() {
        let data = [0x21, 0x42];
        let result = op_arg(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ARG(0x42)));
    }

    #[test]
    fn test_ins_enter_exact_match() {
        let data = [0x3, 0x42, 0x0, 0x0, 0x0];
        let result = ins(&data);
        assert_eq!(result, IResult::Done(&b""[..], Instruction::ENTER(0x42)));
    }

    #[test]
    fn test_qvm_file() {
        let data = include_bytes!("../assets/mod.qvm");
        let result = qvm(data);
        let expected = QVM {
            code: vec![
                Instruction::ENTER(8),
                Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
                Instruction::LEAVE(8),
                Instruction::PUSH,
                Instruction::LEAVE(8),
            ],
            data: vec![
                0
            ],
            lit: vec![],
            bss_length: 65536,
        };
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn test_ins_file() {
        let data = include_bytes!("../assets/mod.qvm");
        named!(ins5<InputSlice,Vec<Instruction>>, count!(ins, 5));
        let result = ins5(&data[32..53]);
        let expected = vec![
            Instruction::ENTER(8),
            Instruction::CONST(4294967295), // TODO: This is actually -1, need to rethink types!
            Instruction::LEAVE(8),
            Instruction::PUSH,
            Instruction::LEAVE(8),
        ];
        assert_eq!(result, IResult::Done(&b""[..], expected));
    }

    #[test]
    fn test_qvm_file_ioq3_baseq3_qagame() {
        let data = include_bytes!("../assets/ioq3/baseq3/vm/qagame.qvm");
        let result = qvm(data);
        // TODO: What to assert here?
    }

}
