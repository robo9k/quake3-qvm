use super::{opcodes, bytecode};

type Input = u8;
type InputSlice<'a> = &'a [Input];

named!(op_break<InputSlice,bytecode::Instruction>,
    value!(
        bytecode::Instruction::BREAK,
        tag!([opcodes::Opcode::BREAK as Input])
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::*;

    #[test]
    fn test_op_break_exact_match() {
        let data = [0x2];
        let result = op_break(&data);
        assert_eq!(result, IResult::Done(&b""[..], bytecode::Instruction::BREAK));
    }

    #[test]
    fn test_op_break_tag_mismatch() {
        let data = [0x0];
        let result = op_break(&data);
        assert_eq!(result, IResult::Error(ErrorKind::Tag));
    }

}
