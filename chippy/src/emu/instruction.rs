#[derive(Debug, PartialEq)]
pub struct TargetSourcePair {
    pub target: u8,
    pub source: u8,
}

#[derive(Debug, PartialEq)]
pub struct RegisterValuePair {
    pub register: u8,
    pub value: u8,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// 0nnn - SYS addr Jump to a machine code routine at nnn.  This instruction is only used on
    /// the old computers on which Chip-8 was originally implemented. It is ignored by modern
    /// interpreters.
    CallMachineCode(u16),

    /// 00E0 - CLS Clear the display.
    ClearDisplay,

    /// 00EE - RET Return from a subroutine.  The interpreter sets the program counter to the
    /// address at the top of the stack, then subtracts 1 from the stack pointer.
    Return,

    /// 1nnn - JP addr Jump to location nnn.  The interpreter sets the program counter to nnn.
    Jump(u16),

    /// 2nnn - CALL addr Call subroutine at nnn.  The interpreter increments the stack pointer,
    /// then puts the current PC on the top of the stack. The PC is then set to nnn.
    Call(u16),

    /// 3xkk - SE Vx, byte Skip next instruction if Vx = kk.  The interpreter compares register Vx
    /// to kk, and if they are equal, increments the program counter by 2.
    SkipIfEq(RegisterValuePair),

    /// 4xkk - SNE Vx, byte Skip next instruction if Vx != kk.  The interpreter compares register
    /// Vx to kk, and if they are not equal, increments the program counter by 2.
    SkipIfNeq(RegisterValuePair),

    /// 5xy0 - SE Vx, Vy Skip next instruction if Vx = Vy.  The interpreter compares register Vx to
    /// register Vy, and if they are equal, increments the program counter by 2.
    SkipIfRegEq(TargetSourcePair),

    /// 6xkk - LD Vx, byte Set Vx = kk.  The interpreter puts the value kk into register Vx.
    SetReg(RegisterValuePair),

    /// 7xkk - ADD Vx, byte Set Vx = Vx + kk.  Adds the value kk to the value of register Vx, then
    /// stores the result in Vx.
    AddValueToReg(RegisterValuePair),

    /// 8xy0 - LD Vx, Vy Set Vx = Vy.  Stores the value of register Vy in register Vx.
    SetRegXToRegY(TargetSourcePair),

    /// 8xy1 - OR Vx, Vy Set Vx = Vx OR Vy.  Performs a bitwise OR on the values of Vx and Vy, then
    /// stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and
    /// if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
    BitXOrY(TargetSourcePair),

    /// 8xy2 - AND Vx, Vy Set Vx = Vx AND Vy.  Performs a bitwise AND on the values of Vx and Vy,
    /// then stores the result in Vx. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is
    /// 0.
    BitXAndY(TargetSourcePair),

    /// 8xy3 - XOR Vx, Vy Set Vx = Vx XOR Vy.  Performs a bitwise exclusive OR on the values of Vx
    /// and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the corresponding bit in the result
    /// is set to 1. Otherwise, it is 0.
    BitXXorY(TargetSourcePair),

    /// 8xy4 - ADD Vx, Vy Set Vx = Vx + Vy, set VF = carry.  The values of Vx and Vy are added
    /// together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
    /// Only the lowest 8 bits of the result are kept, and stored in Vx.
    AddYToX(TargetSourcePair),

    /// 8xy5 - SUB Vx, Vy Set Vx = Vx - Vy, set VF = NOT borrow.  If Vx > Vy, then VF is set to 1,
    /// otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
    SubYFromX(TargetSourcePair),

    /// 8xy6 - SHR Vx {, Vy} Set Vx = Vx SHR 1.  If the least-significant bit of Vx is 1, then VF
    /// is set to 1, otherwise 0. Then Vx is divided by 2. NOTE: there is no information on what y
    /// is set to
    ShiftRight(TargetSourcePair),

    /// 8xy7 - SUBN Vx, Vy Set Vx = Vy - Vx, set VF = NOT borrow.  If Vy > Vx, then VF is set to 1,
    /// otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    SubXFromYIntoX(TargetSourcePair),

    /// 8xyE - SHL Vx {, Vy} Set Vx = Vx SHL 1.  If the most-significant bit of Vx is 1, then VF is
    /// set to 1, otherwise to 0. Then Vx is multiplied by 2. NOTE: there is no information on what
    /// y is set to
    ShiftLeft(TargetSourcePair),

    /// 9xy0 - SNE Vx, Vy Skip next instruction if Vx != Vy.  The values of Vx and Vy are compared,
    /// and if they are not equal, the program counter is increased by 2.
    SkipIfDifferent(TargetSourcePair),

    /// Annn - LD I, addr Set I = nnn.  The value of register I is set to nnn.
    SetI(u16),

    /// Bnnn - JP V0, addr Jump to location nnn + V0.  The program counter is set to nnn plus the
    /// value of V0.
    JumpNPlusPC(u16),

    /// Cxkk - RND Vx, byte Set Vx = random byte AND kk.  The interpreter generates a random number
    /// from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See
    /// instruction 8xy2 for more information on AND.
    Random(RegisterValuePair),

    /// Dxyn - DRW Vx, Vy, nibble Display n-byte sprite starting at memory location I at (Vx, Vy),
    /// set VF = collision.  The interpreter reads n bytes from memory, starting at the address
    /// stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
    /// Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is
    /// set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside
    /// the coordinates of the display, it wraps around to the opposite side of the screen. See
    /// instruction 8xy3 for more information on XOR, and section 2.4, Display, for more
    /// information on the Chip-8 screen and sprites.
    Draw { x: u8, y: u8, n: u8 }, // TODO

    /// Ex9E - SKP Vx Skip next instruction if key with the value of Vx is pressed.  Checks the
    /// keyboard, and if the key corresponding to the value of Vx is currently in the down
    /// position, PC is increased by 2.
    SkipIfKeyPressed(u8),

    /// ExA1 - SKNP Vx Skip next instruction if key with the value of Vx is not pressed.  Checks
    /// the keyboard, and if the key corresponding to the value of Vx is currently in the up
    /// position, PC is increased by 2.
    SkipIfNotKeyPressed(u8),

    /// Fx07 - LD Vx, DT Set Vx = delay timer value.  The value of DT is placed into Vx.
    SetXAsDT(u8),

    /// Fx0A - LD Vx, K Wait for a key press, store the value of the key in Vx.  All execution
    /// stops until a key is pressed, then the value of that key is stored in Vx.
    WaitInputStoreIn(u8),

    /// Fx15 - LD DT, Vx Set delay timer = Vx.  DT is set equal to the value of Vx.
    SetDTAsX(u8),

    /// Fx18 - LD ST, Vx Set sound timer = Vx.  ST is set equal to the value of Vx.
    SetSTAsX(u8),

    /// Fx1E - ADD I, Vx Set I = I + Vx.  The values of I and Vx are added, and the results are
    /// stored in I.
    AddXToI(u8),

    /// Fx29 - LD F, Vx Set I = location of sprite for digit Vx.  The value of I is set to the
    /// location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4,
    /// Display, for more information on the Chip-8 hexadecimal font.
    SetIToFontSprite(u8),

    /// Fx33 - LD B, Vx Store BCD representation of Vx in memory locations I, I+1, and I+2.  The
    /// interpreter takes the decimal value of Vx, and places the hundreds digit in memory at
    /// location in I, the tens digit at location I+1, and the ones digit at location I+2.
    StoreBCD(u8),

    /// Fx55 - LD [I], Vx Store registers V0 through Vx in memory starting at location I.  The
    /// interpreter copies the values of registers V0 through Vx into memory, starting at the
    /// address in I.
    DumpRegisters(u8),

    /// Fx65 - LD Vx, [I] Read registers V0 through Vx from memory starting at location I.  The
    /// interpreter reads values from memory starting at location I into registers V0 through Vx.
    LoadRegisters(u8),

    /// Unknown opcode
    Invalid(u16),
}

fn as_ts_pair(target: u8, source: u8) -> TargetSourcePair {
    TargetSourcePair { target, source }
}

fn as_rv_pair(register: u8, c1: u8, c2: u8) -> RegisterValuePair {
    RegisterValuePair {
        register,
        value: (c1 << 4) | c2,
    }
}

fn as_nnn(opcode: u16) -> u16 {
    opcode & 0xFFF
}

fn as_nibble_array(opcode: u16) -> [u8; 4] {
    let first = ((opcode & 0xF000) >> 12) as u8;
    let second = ((opcode & 0x0F00) >> 8) as u8;
    let third = ((opcode & 0x00F0) >> 4) as u8;
    let fourth = (opcode & 0x000F) as u8;
    [first, second, third, fourth]
}

fn pack_xkk(rv: &RegisterValuePair) -> u16 {
    (((rv.register & 0xF) as u16) << 8) + (rv.value as u16)
}

fn pack_xyn(x: u8, y: u8, n: u8) -> u16 {
    ((x as u16) << 8) + ((y as u16) << 4) + (n as u16)
}

fn pack_tsn(ts: &TargetSourcePair, n: u8) -> u16 {
    pack_xyn(ts.target, ts.source, n)
}

impl Instruction {
    pub fn parse(opcode: u16) -> Instruction {
        let nibbles = as_nibble_array(opcode);
        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => Instruction::ClearDisplay,
            [0x0, 0x0, 0xE, 0xE] => Instruction::Return,
            [0x0, _, _, _] => Instruction::CallMachineCode(as_nnn(opcode)),
            [0x1, _, _, _] => Instruction::Jump(as_nnn(opcode)),
            [0x2, _, _, _] => Instruction::Call(as_nnn(opcode)),
            [0x3, register, c1, c2] => Instruction::SkipIfEq(as_rv_pair(register, c1, c2)),
            [0x4, register, c1, c2] => Instruction::SkipIfNeq(as_rv_pair(register, c1, c2)),
            [0x5, x, y, 0x0] => Instruction::SkipIfRegEq(as_ts_pair(x, y)),
            [0x6, register, c1, c2] => Instruction::SetReg(as_rv_pair(register, c1, c2)),
            [0x7, register, c1, c2] => Instruction::AddValueToReg(as_rv_pair(register, c1, c2)),
            [0x8, x, y, 0x0] => Instruction::SetRegXToRegY(as_ts_pair(x, y)),
            [0x8, x, y, 0x1] => Instruction::BitXOrY(as_ts_pair(x, y)),
            [0x8, x, y, 0x2] => Instruction::BitXAndY(as_ts_pair(x, y)),
            [0x8, x, y, 0x3] => Instruction::BitXXorY(as_ts_pair(x, y)),
            [0x8, x, y, 0x4] => Instruction::AddYToX(as_ts_pair(x, y)),
            [0x8, x, y, 0x5] => Instruction::SubYFromX(as_ts_pair(x, y)),
            [0x8, x, y, 0x6] => Instruction::ShiftRight(as_ts_pair(x, y)),
            [0x8, x, y, 0x7] => Instruction::SubXFromYIntoX(as_ts_pair(x, y)),
            [0x8, x, y, 0xE] => Instruction::ShiftLeft(as_ts_pair(x, y)),
            [0x9, x, y, 0x0] => Instruction::SkipIfDifferent(as_ts_pair(x, y)),
            [0xA, _, _, _] => Instruction::SetI(as_nnn(opcode)),
            [0xB, _, _, _] => Instruction::JumpNPlusPC(as_nnn(opcode)),
            [0xC, register, c1, c2] => Instruction::Random(as_rv_pair(register, c1, c2)),
            [0xD, x, y, n] => Instruction::Draw { x, y, n },
            [0xE, x, 0x9, 0xE] => Instruction::SkipIfKeyPressed(x),
            [0xE, x, 0xA, 0x1] => Instruction::SkipIfNotKeyPressed(x),
            [0xF, x, 0x0, 0x7] => Instruction::SetXAsDT(x),
            [0xF, x, 0x0, 0xA] => Instruction::WaitInputStoreIn(x),
            [0xF, x, 0x1, 0x5] => Instruction::SetDTAsX(x),
            [0xF, x, 0x1, 0x8] => Instruction::SetSTAsX(x),
            [0xF, x, 0x1, 0xE] => Instruction::AddXToI(x),
            [0xF, x, 0x2, 0x9] => Instruction::SetIToFontSprite(x),
            [0xF, x, 0x3, 0x3] => Instruction::StoreBCD(x),
            [0xF, x, 0x5, 0x5] => Instruction::DumpRegisters(x),
            [0xF, x, 0x6, 0x5] => Instruction::LoadRegisters(x),
            _ => Instruction::Invalid(opcode),
        }
    }

    /// Output instruction as asm
    /// Assembily output based on [cowgod's instructions](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#3.1)
    pub fn to_asm(&self) -> String {
        match self {
            Instruction::CallMachineCode(addr) => {
                format!("sys 0x{:03X}", addr)
            }
            Instruction::ClearDisplay => {
                format!("cls")
            }
            Instruction::Return => {
                format!("ret")
            }
            Instruction::Jump(addr) => {
                format!("jp 0x{:03X}", addr)
            }
            Instruction::Call(addr) => {
                format!("call 0x{:03X}", addr)
            }
            Instruction::SkipIfEq(RegisterValuePair { register, value }) => {
                format!("se v{:X}, 0x{:02X}", register, value)
            }
            Instruction::SkipIfNeq(RegisterValuePair { register, value }) => {
                format!("sne v{:X}, 0x{:02X}", register, value)
            }
            Instruction::SkipIfRegEq(TargetSourcePair { target, source }) => {
                format!("se v{:X}, v{:X}", target, source)
            }
            Instruction::SetReg(RegisterValuePair { register, value }) => {
                format!("ld v{:X}, 0x{:02X}", register, value)
            }
            Instruction::AddValueToReg(RegisterValuePair { register, value }) => {
                format!("add v{:X}, 0x{:02X}", register, value)
            }
            Instruction::SetRegXToRegY(TargetSourcePair { target, source }) => {
                format!("ld v{:X}, v{:X}", target, source)
            }
            Instruction::BitXOrY(TargetSourcePair { target, source }) => {
                format!("or v{:X}, v{:X}", target, source)
            }
            Instruction::BitXAndY(TargetSourcePair { target, source }) => {
                format!("and v{:X}, v{:X}", target, source)
            }
            Instruction::BitXXorY(TargetSourcePair { target, source }) => {
                format!("xor v{:X}, v{:X}", target, source)
            }
            Instruction::AddYToX(TargetSourcePair { target, source }) => {
                format!("add v{:X}, v{:X}", target, source)
            }
            Instruction::SubYFromX(TargetSourcePair { target, source }) => {
                format!("sub v{:X}, v{:X}", target, source)
            }
            Instruction::ShiftRight(TargetSourcePair { target, source }) => {
                format!("shr v{:X}", target)
            }
            Instruction::SubXFromYIntoX(TargetSourcePair { target, source }) => {
                format!("subn v{:X}, v{:X}", target, source)
            }
            Instruction::ShiftLeft(TargetSourcePair { target, source }) => {
                format!("shl v{:X}", target)
            }
            Instruction::SkipIfDifferent(TargetSourcePair { target, source }) => {
                format!("sne v{:X}, v{:X}", target, source)
            }
            Instruction::SetI(addr) => {
                format!("ld i, 0x{:03X}", addr)
            }
            Instruction::JumpNPlusPC(addr) => {
                format!("jp v0, 0x{:03X}", addr)
            }
            Instruction::Random(RegisterValuePair { register, value }) => {
                format!("rnd v{:X}, 0x{:02X}", register, value)
            }
            Instruction::Draw { x, y, n } => {
                format!("drw v{:X}, v{:X}, 0x{:X}", x, y, n)
            }
            Instruction::SkipIfKeyPressed(register) => {
                format!("skp v{:X}", register)
            }
            Instruction::SkipIfNotKeyPressed(register) => {
                format!("sknp v{:X}", register)
            }
            Instruction::SetXAsDT(register) => {
                format!("ld v{:x}, dt", register)
            }
            Instruction::WaitInputStoreIn(register) => {
                format!("ld v{:x}, k", register)
            }
            Instruction::SetDTAsX(register) => {
                format!("ld dt, v{:x}", register)
            }
            Instruction::SetSTAsX(register) => {
                format!("ld st, v{:x}", register)
            }
            Instruction::AddXToI(register) => {
                format!("add i, v{:x}", register)
            }
            Instruction::SetIToFontSprite(register) => {
                format!("ld f, v{:x}", register)
            }
            Instruction::StoreBCD(register) => {
                format!("ld b, v{:x}", register)
            }
            Instruction::DumpRegisters(register) => {
                format!("ld [i], v{:x}", register)
            }
            Instruction::LoadRegisters(register) => {
                format!("ld v{:x}, [i]", register)
            }
            Instruction::Invalid(value) => {
                format!("raw 0x{:04X}", value)
            }
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Instruction::CallMachineCode(addr) => (0x0u16 << 12) + addr,
            Instruction::ClearDisplay => 0x00E0,
            Instruction::Return => 0x00EE,
            Instruction::Jump(addr) => (0x1u16 << 12) + addr,
            Instruction::Call(addr) => (0x2u16 << 12) + addr,
            Instruction::SkipIfEq(rv) => (0x3u16 << 12) + pack_xkk(rv),
            Instruction::SkipIfNeq(rv) => (0x4u16 << 12) + pack_xkk(rv),
            Instruction::SkipIfRegEq(ts) => (0x5u16 << 12) + pack_tsn(ts, 0),
            Instruction::SetReg(rv) => (0x6u16 << 12) + pack_xkk(rv),
            Instruction::AddValueToReg(rv) => (0x7u16 << 12) + pack_xkk(rv),
            Instruction::SetRegXToRegY(ts) => (0x8u16 << 12) + pack_tsn(ts, 0),
            Instruction::BitXOrY(ts) => (0x8u16 << 12) + pack_tsn(ts, 1),
            Instruction::BitXAndY(ts) => (0x8u16 << 12) + pack_tsn(ts, 2),
            Instruction::BitXXorY(ts) => (0x8u16 << 12) + pack_tsn(ts, 3),
            Instruction::AddYToX(ts) => (0x8u16 << 12) + pack_tsn(ts, 4),
            Instruction::SubYFromX(ts) => (0x8u16 << 12) + pack_tsn(ts, 5),
            Instruction::ShiftRight(ts) => (0x8u16 << 12) + pack_tsn(ts, 6),
            Instruction::SubXFromYIntoX(ts) => (0x8u16 << 12) + pack_tsn(ts, 7),
            Instruction::ShiftLeft(ts) => (0x8u16 << 12) + pack_tsn(ts, 0xE),
            Instruction::SkipIfDifferent(ts) => (0x9u16 << 12) + pack_tsn(ts, 0),
            Instruction::SetI(addr) => (0xAu16 << 12) + addr,
            Instruction::JumpNPlusPC(addr) => (0xBu16 << 12) + addr,
            Instruction::Random(rv) => (0xCu16 << 12) + pack_xkk(rv),
            Instruction::Draw { x, y, n } => (0xDu16 << 12) + pack_xyn(*x, *y, *n),
            Instruction::SkipIfKeyPressed(register) => {
                (0xEu16 << 12) + pack_xyn(*register, 0x9, 0xE)
            }
            Instruction::SkipIfNotKeyPressed(register) => {
                (0xEu16 << 12) + pack_xyn(*register, 0xA, 0x1)
            }
            Instruction::SetXAsDT(register) => (0xFu16 << 12) + pack_xyn(*register, 0x0, 0x7),
            Instruction::WaitInputStoreIn(register) => {
                (0xFu16 << 12) + pack_xyn(*register, 0x0, 0xA)
            }
            Instruction::SetDTAsX(register) => (0xFu16 << 12) + pack_xyn(*register, 0x1, 0x5),
            Instruction::SetSTAsX(register) => (0xFu16 << 12) + pack_xyn(*register, 0x1, 0x8),
            Instruction::AddXToI(register) => (0xFu16 << 12) + pack_xyn(*register, 0x1, 0xE),
            Instruction::SetIToFontSprite(register) => {
                (0xFu16 << 12) + pack_xyn(*register, 0x2, 0x9)
            }
            Instruction::StoreBCD(register) => (0xFu16 << 12) + pack_xyn(*register, 0x3, 0x3),
            Instruction::DumpRegisters(register) => (0xFu16 << 12) + pack_xyn(*register, 0x5, 0x5),
            Instruction::LoadRegisters(register) => (0xFu16 << 12) + pack_xyn(*register, 0x6, 0x5),
            Instruction::Invalid(code) => *code,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_nibble_array() {
        let result = as_nibble_array(0xDEAF);
        assert_eq!(result, [0xD, 0xE, 0xA, 0xF]);
    }

    #[test]
    fn call_machine_code() {
        assert_eq!(
            Instruction::CallMachineCode(0xDEA),
            Instruction::parse(0x0DEA)
        );
    }

    #[test]
    fn clear_display() {
        assert_eq!(Instruction::ClearDisplay, Instruction::parse(0x00E0));
    }

    #[test]
    fn return_code() {
        assert_eq!(Instruction::Return, Instruction::parse(0x00EE));
    }

    #[test]
    fn jump() {
        assert_eq!(Instruction::Jump(0xDEA), Instruction::parse(0x1DEA));
    }

    #[test]
    fn call() {
        assert_eq!(Instruction::Call(0xDEA), Instruction::parse(0x2DEA));
    }

    #[test]
    fn skip_if_equal() {
        assert_eq!(
            Instruction::SkipIfEq(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Instruction::parse(0x3ABB)
        );
    }

    #[test]
    fn skip_if_not_equal() {
        assert_eq!(
            Instruction::SkipIfNeq(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Instruction::parse(0x4ABB)
        );
    }

    #[test]
    fn skip_if_reqister_equal() {
        assert_eq!(
            Instruction::SkipIfRegEq(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x5AB0)
        );
    }

    #[test]
    fn set_register() {
        assert_eq!(
            Instruction::SetReg(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Instruction::parse(0x6ABB)
        );
    }

    #[test]
    fn add_value_to_register() {
        assert_eq!(
            Instruction::AddValueToReg(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Instruction::parse(0x7ABB)
        );
    }

    #[test]
    fn set_register_x_to_y() {
        assert_eq!(
            Instruction::SetRegXToRegY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB0)
        );
    }

    #[test]
    fn bit_x_or_y() {
        assert_eq!(
            Instruction::BitXOrY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB1)
        );
    }

    #[test]
    fn bit_x_and_y() {
        assert_eq!(
            Instruction::BitXAndY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB2)
        );
    }

    #[test]
    fn bit_x_xor_y() {
        assert_eq!(
            Instruction::BitXXorY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB3)
        );
    }

    #[test]
    fn and_y_to_x() {
        assert_eq!(
            Instruction::AddYToX(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB4)
        );
    }

    #[test]
    fn sub_y_from_x() {
        assert_eq!(
            Instruction::SubYFromX(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB5)
        );
    }

    #[test]
    fn shift_right() {
        assert_eq!(
            Instruction::ShiftRight(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB6)
        );
    }

    #[test]
    fn sub_x_from_y_into_x() {
        assert_eq!(
            Instruction::SubXFromYIntoX(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8AB7)
        );
    }

    #[test]
    fn shift_left() {
        assert_eq!(
            Instruction::ShiftLeft(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x8ABE)
        );
    }

    #[test]
    fn skip_if_different() {
        assert_eq!(
            Instruction::SkipIfDifferent(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Instruction::parse(0x9AB0)
        );
    }

    #[test]
    fn set_i() {
        assert_eq!(Instruction::SetI(0xDEA), Instruction::parse(0xADEA));
    }

    #[test]
    fn jump_n_plus_pc() {
        assert_eq!(Instruction::JumpNPlusPC(0xDEA), Instruction::parse(0xBDEA));
    }

    #[test]
    fn random() {
        assert_eq!(
            Instruction::Random(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Instruction::parse(0xCABB)
        );
    }

    #[test]
    fn draw() {
        assert_eq!(
            Instruction::Draw {
                x: 0xA,
                y: 0xB,
                n: 0xC,
            },
            Instruction::parse(0xDABC)
        );
    }

    #[test]
    fn skip_if_key_pressed() {
        assert_eq!(
            Instruction::SkipIfKeyPressed(0xA),
            Instruction::parse(0xEA9E)
        );
    }

    #[test]
    fn skip_if_not_key_pressed() {
        assert_eq!(
            Instruction::SkipIfNotKeyPressed(0xA),
            Instruction::parse(0xEAA1)
        );
    }

    #[test]
    fn set_x_as_dt() {
        assert_eq!(Instruction::SetXAsDT(0xA), Instruction::parse(0xFA07));
    }

    #[test]
    fn wait_input_store_in() {
        assert_eq!(
            Instruction::WaitInputStoreIn(0xA),
            Instruction::parse(0xFA0A)
        );
    }

    #[test]
    fn set_dt_as_x() {
        assert_eq!(Instruction::SetDTAsX(0xA), Instruction::parse(0xFA15));
    }

    #[test]
    fn set_st_as_x() {
        assert_eq!(Instruction::SetSTAsX(0xA), Instruction::parse(0xFA18));
    }

    #[test]
    fn add_x_to_i() {
        assert_eq!(Instruction::AddXToI(0xA), Instruction::parse(0xFA1E));
    }

    #[test]
    fn set_i_to_font_sprite() {
        assert_eq!(
            Instruction::SetIToFontSprite(0xA),
            Instruction::parse(0xFA29)
        );
    }

    #[test]
    fn store_bcd() {
        assert_eq!(Instruction::StoreBCD(0xA), Instruction::parse(0xFA33));
    }

    #[test]
    fn dump_registers() {
        assert_eq!(Instruction::DumpRegisters(0xA), Instruction::parse(0xFA55));
    }

    #[test]
    fn load_registers() {
        assert_eq!(Instruction::LoadRegisters(0xA), Instruction::parse(0xFA65));
    }

    #[test]
    fn asm_output() {
        let pairs = vec![
            (0x00E0, "cls"),
            (0x00EE, "ret"),
            (0x0246, "sys 0x246"),
            (0x1246, "jp 0x246"),
            (0x2357, "call 0x357"),
            (0x32DE, "se v2, 0xDE"),
            (0x42DE, "sne v2, 0xDE"),
            (0x5210, "se v2, v1"),
            (0x6218, "ld v2, 0x18"),
            (0x70E3, "add v0, 0xE3"),
            (0x8120, "ld v1, v2"),
            (0x8121, "or v1, v2"),
            (0x8122, "and v1, v2"),
            (0x8123, "xor v1, v2"),
            (0x8124, "add v1, v2"),
            (0x8125, "sub v1, v2"),
            (0x8126, "shr v1"),
            (0x8127, "subn v1, v2"),
            (0x812E, "shl v1"),
            (0x93E0, "sne v3, vE"),
            (0xA123, "ld i, 0x123"),
            (0xB123, "jp v0, 0x123"),
            (0xC123, "rnd v1, 0x23"),
            (0xD123, "drw v1, v2, 0x3"),
            (0xE19E, "skp v1"),
            (0xE1A1, "sknp v1"),
            (0xF107, "ld v1, dt"),
            (0xF10A, "ld v1, k"),
            (0xF115, "ld dt, v1"),
            (0xF118, "ld st, v1"),
            (0xF11E, "add i, v1"),
            (0xF129, "ld f, v1"),
            (0xF133, "ld b, v1"),
            (0xF155, "ld [i], v1"),
            (0xF165, "ld v1, [i]"),
            (0xF169, "raw 0xF169"),
        ];

        for (code, result) in pairs {
            let instruction = Instruction::parse(code);
            let actual = instruction.to_asm();
            assert_eq!(actual, result);
        }
    }

    #[test]
    fn code_to_u16() {
        // NOTE: for 0x8xy6 (shift left) 0x8xyE (shift right) dont store y to set them to 0 for the test
        let code_list = vec![
            0x00E0, 0x00EE, 0x0246, 0x1246, 0x2357, 0x32DE, 0x42DE, 0x5210, 0x6218, 0x70E3, 0x8120,
            0x8121, 0x8122, 0x8123, 0x8124, 0x8125, 0x8106, 0x8127, 0x810E, 0x93E0, 0xA123, 0xB123,
            0xC123, 0xD123, 0xE19E, 0xE1A1, 0xF107, 0xF10A, 0xF115, 0xF118, 0xF11E, 0xF129, 0xF133,
            0xF155, 0xF165, 0xF169,
        ];

        for code in code_list {
            let instruction = Instruction::parse(code);
            let result = instruction.to_u16();
            assert_eq!(result, code);
        }
    }

    #[test]
    fn packing_xkk() {
        let rv = RegisterValuePair {
            register: 0xA,
            value: 0xBB,
        };
        let result = 0x6ABB;
        let actual = (0x6u16 << 12) + pack_xkk(&rv);

        assert_eq!(actual, result);
    }

    #[test]
    fn packing_tsn() {
        let ts = TargetSourcePair {
            target: 0xA,
            source: 0xB,
        };
        let result = 0x8AB2;
        let actual = (08u16 << 12) + pack_tsn(&ts, 2);

        assert_eq!(actual, result);
    }
}
