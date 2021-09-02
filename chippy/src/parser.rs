use crate::emu::opcode::Opcode;
use thiserror::Error;

pub type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct ByteCodeIter<'a> {
    slice: &'a [u8],
    index: usize,
    len: usize,
}

impl<'a> ByteCodeIter<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        assert!(
            slice.len() % 2 == 0,
            "ByteCode must be an even array as opcodes are 2 u8"
        );
        Self {
            slice,
            index: 0usize,
            len: slice.len() / 2,
        }
    }
}

impl<'a> Iterator for ByteCodeIter<'a> {
    type Item = u16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len {
            return None;
        }

        let code =
            ((self.slice[self.index * 2] as u16) << 8) + self.slice[self.index * 2 + 1] as u16;
        self.index += 1;

        Some(code)
    }
}

pub fn from_asm(program: &str) -> ParseResult<Vec<Opcode>> {
    Ok(vec![])
}

pub fn to_bin(opcodes: &[Opcode]) -> ParseResult<Vec<u8>> {
    Ok(opcodes
        .iter()
        .flat_map(|code| code.to_u16().to_be_bytes())
        .collect())
}

pub fn to_asm(tokens: &[u8]) -> String {
    let instructions: Vec<String> = ByteCodeIter::new(tokens)
        .map(|code| Opcode::parse(code).to_asm())
        .collect();

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

    #[test]
    fn bin_iterator() {
        let program = vec![0x12, 0x23, 0x34, 0x45, 0x56, 0x67];
        let mut iter = ByteCodeIter::new(&program);
        assert_eq!(Some(0x1223), iter.next());
        assert_eq!(Some(0x3445), iter.next());
        assert_eq!(Some(0x5667), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    #[should_panic]
    fn bin_iterator_panic_odd_slice() {
        // This program is invalid as it is odd length and cannot construct a opcode
        let program = vec![0x12, 0x23, 0x34];

        // This should panic
        let iter = ByteCodeIter::new(&program);
    }

    // #[test]
    // fn opcodes_to_bin() {}
}
