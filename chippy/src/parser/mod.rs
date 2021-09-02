use crate::emu::{instruction::Instruction, iter::ByteCodeIter};
use crate::parser::error::ParseResult;

pub mod error;

pub fn from_asm(program: &str) -> ParseResult<Vec<Instruction>> {
    Ok(vec![])
}

pub fn from_bytecode(bytecode: &[u8]) -> ParseResult<Vec<Instruction>> {
    Ok(ByteCodeIter::new(bytecode)
        .map(|code| Instruction::parse(code))
        .collect())
}

pub fn to_bytecode(instructions: &[Instruction]) -> ParseResult<Vec<u8>> {
    Ok(instructions
        .iter()
        .flat_map(|code| code.to_u16().to_be_bytes())
        .collect())
}

pub fn to_asm(instructions: &[Instruction]) -> ParseResult<String> {
    let lines: Vec<String> = instructions
        .iter()
        .map(|instruction| instruction.to_asm())
        .collect();

    Ok(format!("{}", lines.join("\n")))
}

#[cfg(test)]
mod tests {
    use crate::emu::instruction::{RegisterValuePair, TargetSourcePair};

    use super::*;

    fn rv(register: u8, value: u8) -> RegisterValuePair {
        RegisterValuePair { register, value }
    }

    fn ts(target: u8, source: u8) -> TargetSourcePair {
        TargetSourcePair { target, source }
    }

    fn get_program() -> Vec<u8> {
        vec![
            0x00, 0xE0, 0x00, 0xEE, 0x02, 0x46, 0x12, 0x46, 0x23, 0x57, 0x32, 0xDE, 0x42, 0xDE,
            0x52, 0x10, 0x62, 0x18, 0x70, 0xE3, 0x81, 0x20, 0x81, 0x21, 0x81, 0x22, 0x81, 0x23,
            0x81, 0x24, 0x81, 0x25, 0x81, 0x26, 0x81, 0x27, 0x81, 0x2E, 0x93, 0xE0, 0xA1, 0x23,
            0xB1, 0x23, 0xC1, 0x23, 0xD1, 0x23, 0xE1, 0x9E, 0xE1, 0xA1, 0xF1, 0x07, 0xF1, 0x0A,
            0xF1, 0x15, 0xF1, 0x18, 0xF1, 0x1E, 0xF1, 0x29, 0xF1, 0x33, 0xF1, 0x55, 0xF1, 0x65,
            0xF1, 0x69,
        ]
    }

    fn get_instructions() -> Vec<Instruction> {
        use Instruction::*;
        vec![
            ClearDisplay,
            Return,
            CallMachineCode(0x246),
            Jump(0x246),
            Call(0x357),
            SkipIfEq(rv(2, 0xDE)),
            SkipIfNeq(rv(2, 0xDE)),
            SkipIfRegEq(ts(2, 1)),
            SetReg(rv(2, 0x18)),
            AddValueToReg(rv(0, 0xE3)),
            SetRegXToRegY(ts(1, 2)),
            BitXOrY(ts(1, 2)),
            BitXAndY(ts(1, 2)),
            BitXXorY(ts(1, 2)),
            AddYToX(ts(1, 2)),
            SubYFromX(ts(1, 2)),
            ShiftRight(ts(1, 2)),
            SubXFromYIntoX(ts(1, 2)),
            ShiftLeft(ts(1, 2)),
            SkipIfDifferent(ts(3, 0xE)),
            SetI(0x123),
            JumpNPlusPC(0x123),
            Random(rv(1, 0x23)),
            Draw { x: 1, y: 2, n: 3 },
            SkipIfKeyPressed(1),
            SkipIfNotKeyPressed(1),
            SetXAsDT(1),
            WaitInputStoreIn(1),
            SetDTAsX(1),
            SetSTAsX(1),
            AddXToI(1),
            SetIToFontSprite(1),
            StoreBCD(1),
            DumpRegisters(1),
            LoadRegisters(1),
            Invalid(0xF169),
        ]
    }

    fn get_asm() -> String {
        String::from(
            r#"cls
ret
sys 0x246
jp 0x246
call 0x357
se v2, 0xDE
sne v2, 0xDE
se v2, v1
ld v2, 0x18
add v0, 0xE3
ld v1, v2
or v1, v2
and v1, v2
xor v1, v2
add v1, v2
sub v1, v2
shr v1
subn v1, v2
shl v1
sne v3, vE
ld i, 0x123
jp v0, 0x123
rnd v1, 0x23
drw v1, v2, 0x3
skp v1
sknp v1
ld v1, dt
ld v1, k
ld dt, v1
ld st, v1
add i, v1
ld f, v1
ld b, v1
ld [i], v1
ld v1, [i]
raw 0xF169"#,
        )
    }

    #[test]
    fn from_bytecode_to_instruction() {
        let program = get_program();
        let instructions = from_bytecode(&program).unwrap();
        let actual = get_instructions();
        assert_eq!(instructions, actual);
    }

    #[test]
    fn from_instructions_to_bytecode() {
        let instructions = get_instructions();
        let bytecode = to_bytecode(&instructions).unwrap();
        let actual = get_program();
        assert_eq!(bytecode, actual);
    }

    #[test]
    fn from_instructions_to_asm() {
        let instruction = get_instructions();
        let asm = to_asm(&&instruction).unwrap();
        let actual = get_asm();
        assert_eq!(asm, actual);
    }
}
