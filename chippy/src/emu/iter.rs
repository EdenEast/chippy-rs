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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytecode_iter_success() {
        let program = vec![0x12, 0x23, 0x34, 0x45, 0x56, 0x67];
        let mut iter = ByteCodeIter::new(&program);
        assert_eq!(Some(0x1223), iter.next());
        assert_eq!(Some(0x3445), iter.next());
        assert_eq!(Some(0x5667), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    #[should_panic]
    fn bytecode_iter_panic_odd_slice() {
        // This program is invalid as it is odd length and cannot construct a opcode
        let program = vec![0x12, 0x23, 0x34];

        // This should panic
        let iter = ByteCodeIter::new(&program);
    }
}
