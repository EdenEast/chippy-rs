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
pub enum Opcode {
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
    /// is set to 1, otherwise 0. Then Vx is divided by 2.
    ShiftRight(u8),

    /// 8xy7 - SUBN Vx, Vy Set Vx = Vy - Vx, set VF = NOT borrow.  If Vy > Vx, then VF is set to 1,
    /// otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    SubXFromYIntoX(TargetSourcePair),

    /// 8xyE - SHL Vx {, Vy} Set Vx = Vx SHL 1.  If the most-significant bit of Vx is 1, then VF is
    /// set to 1, otherwise to 0. Then Vx is multiplied by 2.
    ShiftLeft(u8),

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

impl Opcode {
    pub fn parse(opcode: u16) -> Opcode {
        let nibbles = as_nibble_array(opcode);
        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => Opcode::ClearDisplay,
            [0x0, 0x0, 0xE, 0xE] => Opcode::Return,
            [0x0, _, _, _] => Opcode::CallMachineCode(as_nnn(opcode)),
            [0x1, _, _, _] => Opcode::Jump(as_nnn(opcode)),
            [0x2, _, _, _] => Opcode::Call(as_nnn(opcode)),
            [0x3, register, c1, c2] => Opcode::SkipIfEq(as_rv_pair(register, c1, c2)),
            [0x4, register, c1, c2] => Opcode::SkipIfNeq(as_rv_pair(register, c1, c2)),
            [0x5, x, y, 0x0] => Opcode::SkipIfRegEq(as_ts_pair(x, y)),
            [0x6, register, c1, c2] => Opcode::SetReg(as_rv_pair(register, c1, c2)),
            [0x7, register, c1, c2] => Opcode::AddValueToReg(as_rv_pair(register, c1, c2)),
            [0x8, x, y, 0x0] => Opcode::SetRegXToRegY(as_ts_pair(x, y)),
            [0x8, x, y, 0x1] => Opcode::BitXOrY(as_ts_pair(x, y)),
            [0x8, x, y, 0x2] => Opcode::BitXAndY(as_ts_pair(x, y)),
            [0x8, x, y, 0x3] => Opcode::BitXXorY(as_ts_pair(x, y)),
            [0x8, x, y, 0x4] => Opcode::AddYToX(as_ts_pair(x, y)),
            [0x8, x, y, 0x5] => Opcode::SubYFromX(as_ts_pair(x, y)),
            [0x8, x, _, 0x6] => Opcode::ShiftRight(x),
            [0x8, x, y, 0x7] => Opcode::SubXFromYIntoX(as_ts_pair(x, y)),
            [0x8, x, _, 0xE] => Opcode::ShiftLeft(x),
            [0x9, x, y, 0x0] => Opcode::SkipIfDifferent(as_ts_pair(x, y)),
            [0xA, _, _, _] => Opcode::SetI(as_nnn(opcode)),
            [0xB, _, _, _] => Opcode::JumpNPlusPC(as_nnn(opcode)),
            [0xC, register, c1, c2] => Opcode::Random(as_rv_pair(register, c1, c2)),
            [0xD, x, y, n] => Opcode::Draw { x, y, n },
            [0xE, x, 0x9, 0xE] => Opcode::SkipIfKeyPressed(x),
            [0xE, x, 0xA, 0x1] => Opcode::SkipIfNotKeyPressed(x),
            [0xF, x, 0x0, 0x7] => Opcode::SetXAsDT(x),
            [0xF, x, 0x0, 0xA] => Opcode::WaitInputStoreIn(x),
            [0xF, x, 0x1, 0x5] => Opcode::SetDTAsX(x),
            [0xF, x, 0x1, 0x8] => Opcode::SetSTAsX(x),
            [0xF, x, 0x1, 0xE] => Opcode::AddXToI(x),
            [0xF, x, 0x2, 0x9] => Opcode::SetIToFontSprite(x),
            [0xF, x, 0x3, 0x3] => Opcode::StoreBCD(x),
            [0xF, x, 0x5, 0x5] => Opcode::DumpRegisters(x),
            [0xF, x, 0x6, 0x5] => Opcode::LoadRegisters(x),
            _ => Opcode::Invalid(opcode),
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
        assert_eq!(Opcode::CallMachineCode(0xDEA), Opcode::parse(0x0DEA));
    }

    #[test]
    fn clear_display() {
        assert_eq!(Opcode::ClearDisplay, Opcode::parse(0x00E0));
    }

    #[test]
    fn return_code() {
        assert_eq!(Opcode::Return, Opcode::parse(0x00EE));
    }

    #[test]
    fn jump() {
        assert_eq!(Opcode::Jump(0xDEA), Opcode::parse(0x1DEA));
    }

    #[test]
    fn call() {
        assert_eq!(Opcode::Call(0xDEA), Opcode::parse(0x2DEA));
    }

    #[test]
    fn skip_if_equal() {
        assert_eq!(
            Opcode::SkipIfEq(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Opcode::parse(0x3ABB)
        );
    }

    #[test]
    fn skip_if_not_equal() {
        assert_eq!(
            Opcode::SkipIfNeq(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Opcode::parse(0x4ABB)
        );
    }

    #[test]
    fn skip_if_reqister_equal() {
        assert_eq!(
            Opcode::SkipIfRegEq(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x5AB0)
        );
    }

    #[test]
    fn set_register() {
        assert_eq!(
            Opcode::SetReg(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Opcode::parse(0x6ABB)
        );
    }

    #[test]
    fn add_value_to_register() {
        assert_eq!(
            Opcode::AddValueToReg(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Opcode::parse(0x7ABB)
        );
    }

    #[test]
    fn set_register_x_to_y() {
        assert_eq!(
            Opcode::SetRegXToRegY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB0)
        );
    }

    #[test]
    fn bit_x_or_y() {
        assert_eq!(
            Opcode::BitXOrY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB1)
        );
    }

    #[test]
    fn bit_x_and_y() {
        assert_eq!(
            Opcode::BitXAndY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB2)
        );
    }

    #[test]
    fn bit_x_xor_y() {
        assert_eq!(
            Opcode::BitXXorY(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB3)
        );
    }

    #[test]
    fn and_y_to_x() {
        assert_eq!(
            Opcode::AddYToX(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB4)
        );
    }

    #[test]
    fn sub_y_from_x() {
        assert_eq!(
            Opcode::SubYFromX(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB5)
        );
    }

    #[test]
    fn shift_right() {
        assert_eq!(Opcode::ShiftRight(0xA), Opcode::parse(0x8AB6));
    }

    #[test]
    fn sub_x_from_y_into_x() {
        assert_eq!(
            Opcode::SubXFromYIntoX(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x8AB7)
        );
    }

    #[test]
    fn shift_left() {
        assert_eq!(Opcode::ShiftLeft(0xA), Opcode::parse(0x8ABE));
    }

    #[test]
    fn skip_if_different() {
        assert_eq!(
            Opcode::SkipIfDifferent(TargetSourcePair {
                target: 0xA,
                source: 0xB,
            }),
            Opcode::parse(0x9AB0)
        );
    }

    #[test]
    fn set_i() {
        assert_eq!(Opcode::SetI(0xDEA), Opcode::parse(0xADEA));
    }

    #[test]
    fn jump_n_plus_pc() {
        assert_eq!(Opcode::JumpNPlusPC(0xDEA), Opcode::parse(0xBDEA));
    }

    #[test]
    fn random() {
        assert_eq!(
            Opcode::Random(RegisterValuePair {
                register: 0xA,
                value: 0xBB,
            }),
            Opcode::parse(0xCABB)
        );
    }

    #[test]
    fn draw() {
        assert_eq!(
            Opcode::Draw {
                x: 0xA,
                y: 0xB,
                n: 0xC,
            },
            Opcode::parse(0xDABC)
        );
    }

    #[test]
    fn skip_if_key_pressed() {
        assert_eq!(Opcode::SkipIfKeyPressed(0xA), Opcode::parse(0xEA9E));
    }

    #[test]
    fn skip_if_not_key_pressed() {
        assert_eq!(Opcode::SkipIfNotKeyPressed(0xA), Opcode::parse(0xEAA1));
    }

    #[test]
    fn set_x_as_dt() {
        assert_eq!(Opcode::SetXAsDT(0xA), Opcode::parse(0xFA07));
    }

    #[test]
    fn wait_input_store_in() {
        assert_eq!(Opcode::WaitInputStoreIn(0xA), Opcode::parse(0xFA0A));
    }

    #[test]
    fn set_dt_as_x() {
        assert_eq!(Opcode::SetDTAsX(0xA), Opcode::parse(0xFA15));
    }

    #[test]
    fn set_st_as_x() {
        assert_eq!(Opcode::SetSTAsX(0xA), Opcode::parse(0xFA18));
    }

    #[test]
    fn add_x_to_i() {
        assert_eq!(Opcode::AddXToI(0xA), Opcode::parse(0xFA1E));
    }

    #[test]
    fn set_i_to_font_sprite() {
        assert_eq!(Opcode::SetIToFontSprite(0xA), Opcode::parse(0xFA29));
    }

    #[test]
    fn store_bcd() {
        assert_eq!(Opcode::StoreBCD(0xA), Opcode::parse(0xFA33));
    }

    #[test]
    fn dump_registers() {
        assert_eq!(Opcode::DumpRegisters(0xA), Opcode::parse(0xFA55));
    }

    #[test]
    fn load_registers() {
        assert_eq!(Opcode::LoadRegisters(0xA), Opcode::parse(0xFA65));
    }
}
