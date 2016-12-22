use super::{opcodes, bytecode};

named!(op_break<&[u8],bytecode::Instruction>,
    do_parse!(
        tag!([opcodes::Opcode::BREAK as u8]) >>
        (bytecode::Instruction::BREAK)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::*;

    #[test]
    fn test_op_break() {
        let data = [0x2];
        let result = op_break(&data);
        assert_eq!(result, IResult::Done(&b""[..], bytecode::Instruction::BREAK));
    }
}
