use crate::emu::opcode::Opcode;

pub fn to_asm(tokens: &[u8]) -> String {
    // Since every opcode is a u16 built with two tokens the instruction are only half
    let mut instructions = Vec::with_capacity(tokens.len() / 2);

    for i in 0..(tokens.len() / 2) {
        let value = ((tokens[i * 2] as u16) << 8) + tokens[i * 2 + 1] as u16;
        let opcode = Opcode::parse(value);
        instructions.push(opcode.to_asm())
    }

    format!("{}", instructions.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bin_to_asm() {
        let program = vec![
            0x00, 0xE0, 0x00, 0xEE, 0x02, 0x46, 0x12, 0x46, 0x23, 0x57, 0x32, 0xDE, 0x42, 0xDE,
            0x52, 0x10, 0x62, 0x18, 0x70, 0xE3, 0x81, 0x20, 0x81, 0x21, 0x81, 0x22, 0x81, 0x23,
            0x81, 0x24, 0x81, 0x25, 0x81, 0x26, 0x81, 0x27, 0x81, 0x2E, 0x93, 0xE0, 0xA1, 0x23,
            0xB1, 0x23, 0xC1, 0x23, 0xD1, 0x23, 0xE1, 0x9E, 0xE1, 0xA1, 0xF1, 0x07, 0xF1, 0x0A,
            0xF1, 0x15, 0xF1, 0x18, 0xF1, 0x1E, 0xF1, 0x29, 0xF1, 0x33, 0xF1, 0x55, 0xF1, 0x65,
            0xF1, 0x69,
        ];
        let actual = concat!(
            "cls\n",
            "ret\n",
            "sys 0x246\n",
            "jp 0x246\n",
            "call 0x357\n",
            "se v2, 0xDE\n",
            "sne v2, 0xDE\n",
            "se v2, v1\n",
            "ld v2, 0x18\n",
            "add v0, 0xE3\n",
            "ld v1, v2\n",
            "or v1, v2\n",
            "and v1, v2\n",
            "xor v1, v2\n",
            "add v1, v2\n",
            "sub v1, v2\n",
            "shr v1\n",
            "subn v1, v2\n",
            "shl v1\n",
            "sne v3, vE\n",
            "ld i, 0x123\n",
            "jp v0, 0x123\n",
            "rnd v1, 0x23\n",
            "drw v1, v2, 0x3\n",
            "skp v1\n",
            "sknp v1\n",
            "ld v1, dt\n",
            "ld v1, k\n",
            "ld dt, v1\n",
            "ld st, v1\n",
            "add i, v1\n",
            "ld f, v1\n",
            "ld b, v1\n",
            "ld [i], v1\n",
            "ld v1, [i]\n",
            "raw 0xF169",
        );
        let result = to_asm(&program);
        assert_eq!(result, actual);
    }
}
