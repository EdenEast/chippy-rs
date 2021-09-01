use crate::opcode::{Opcode, RegisterValuePair, TargetSourcePair};
use byteorder::{BigEndian, ReadBytesExt};

const INITIAL_PROGRAM_COUNTER: u16 = 0x200;
const MEMORY_SIZE: usize = 4096;
const MEMORY_START: usize = 512;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 16;

type Register = u8;
type StackEntry = u16;

pub enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

fn skip_if(condition: bool) -> ProgramCounter {
    if condition {
        ProgramCounter::Skip
    } else {
        ProgramCounter::Next
    }
}

pub struct Vm {
    memory: [u8; MEMORY_SIZE],
    registers: [Register; REGISTER_SIZE],
    stack: [StackEntry; STACK_SIZE],
    stack_pointer: usize,
    index: u16,
    program_counter: u16,
}

impl Vm {
    pub fn new() -> Self {
        let memory = [0; MEMORY_SIZE];

        Self {
            memory,
            registers: [0; REGISTER_SIZE],
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            index: 0,
            program_counter: INITIAL_PROGRAM_COUNTER,
        }
    }

    pub fn load(&mut self, buffer: Vec<u8>) {
        for (index, value) in buffer.iter().enumerate() {
            self.memory[index + MEMORY_START] = *value;
        }
    }

    pub fn reset(&mut self) {
        for index in MEMORY_START..MEMORY_SIZE {
            self.memory[index] = 0;
        }

        self.registers = [0; REGISTER_SIZE];
        self.stack = [0; STACK_SIZE];
        self.stack_pointer = 0;
        self.index = 0;
        self.program_counter = INITIAL_PROGRAM_COUNTER;
    }

    pub fn cycle(&mut self) {
        let position = self.program_counter as usize;
        let mut parts = &self.memory[position..position + 2];
        let opcode = parts.read_u16::<BigEndian>().unwrap();

        self.program_counter = match self.execute_opcode(opcode) {
            ProgramCounter::Next => self.program_counter + 2,
            ProgramCounter::Skip => self.program_counter + 4,
            ProgramCounter::Jump(addr) => addr,
        };
    }

    pub fn execute_opcode(&mut self, opcode: u16) -> ProgramCounter {
        match Opcode::parse(opcode) {
            Opcode::CallMachineCode(_) => {
                ProgramCounter::Next // TODO
            }
            Opcode::ClearDisplay => {
                ProgramCounter::Next // TODO
            }
            Opcode::Return => ProgramCounter::Jump(self.pop_stack()),
            Opcode::Jump(addr) => ProgramCounter::Jump(addr),
            Opcode::Call(addr) => {
                self.push_stack();
                ProgramCounter::Jump(addr)
            }
            Opcode::SkipIfEq(RegisterValuePair { register, value }) => {
                skip_if(self.get_register(register) == value)
            }
            Opcode::SkipIfNeq(RegisterValuePair { register, value }) => {
                skip_if(self.get_register(register) != value)
            }
            Opcode::SkipIfRegEq(TargetSourcePair { target, source }) => {
                skip_if(self.get_register(target) == self.get_register(source))
            }
            Opcode::SetReg(RegisterValuePair { register, value }) => {
                self.set_register(register, value);
                ProgramCounter::Next
            }
            Opcode::AddValueToReg(RegisterValuePair { register, value }) => {
                let (sum, _) = self.get_register(register).overflowing_add(value);
                self.set_register(register, sum);
                ProgramCounter::Next
            }
            Opcode::SetRegXToRegY(TargetSourcePair { target, source }) => {
                self.set_register(target, self.get_register(source));
                ProgramCounter::Next
            }
            Opcode::BitXOrY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) | self.get_register(source);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Opcode::BitXAndY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) & self.get_register(source);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Opcode::BitXXorY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) ^ self.get_register(source);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Opcode::AddYToX(TargetSourcePair { target, source }) => {
                let (result, did_overflow) = self
                    .get_register(target)
                    .overflowing_add(self.get_register(source));
                self.set_vf_confitional(did_overflow);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Opcode::SubYFromX(TargetSourcePair { target, source }) => {
                let (result, did_overflow) = self
                    .get_register(target)
                    .overflowing_sub(self.get_register(source));
                self.set_vf_confitional(!did_overflow);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Opcode::ShiftRight(register) => {
                let value = self.get_register(register);
                self.set_vf_register(value & 0xF);
                self.set_register(register, value >> 1);
                ProgramCounter::Next
            }
            Opcode::SubXFromYIntoX(TargetSourcePair { target, source }) => {
                let (result, did_overflow) = self
                    .get_register(source)
                    .overflowing_sub(self.get_register(target));
                self.set_vf_confitional(!did_overflow);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Opcode::ShiftLeft(register) => {
                let value = self.get_register(register);
                self.set_vf_register(value >> 7);
                self.set_register(register, value << 1);
                ProgramCounter::Next
            }
            Opcode::SkipIfDifferent(TargetSourcePair { target, source }) => {
                skip_if(self.get_register(target) != self.get_register(source))
            }
            Opcode::SetI(value) => {
                self.index = value;
                ProgramCounter::Next
            }
            Opcode::JumpNPlusPC(addr) => ProgramCounter::Jump(addr + self.get_register(0x0) as u16),
            Opcode::Random(RegisterValuePair { register, value }) => {
                // TODO: get random number between 0, 255
                let random = 0x5d;
                self.set_register(register, random & value);
                ProgramCounter::Next
            }
            Opcode::Draw => {
                unimplemented!() // TODO
            }
            Opcode::SkipIfKeyPressed(_) => {
                unimplemented!() // TODO
            }
            Opcode::SkipIfNotKeyPressed(_) => {
                unimplemented!() // TODO
            }
            Opcode::SetXAsDT(_) => {
                unimplemented!() // TODO
            }
            Opcode::WaitInputStoreIn(_) => {
                unimplemented!() // TODO
            }
            Opcode::SetDTAsX(_) => {
                unimplemented!() // TODO
            }
            Opcode::SetSTAsX(_) => {
                unimplemented!() // TODO
            }
            Opcode::AddXToI(register) => {
                let (result, _) = self
                    .index
                    .overflowing_add(self.get_register(register) as u16);
                self.index = result;
                ProgramCounter::Next
            }
            Opcode::SetIToFontSprite(_) => {
                unimplemented!() // TODO
            }
            Opcode::StoreBCD(_) => {
                unimplemented!() // TODO
            }
            Opcode::DumpRegisters(_) => {
                unimplemented!() // TODO
            }
            Opcode::LoadRegisters(_) => {
                unimplemented!() // TODO
            }
            Opcode::Invalid(_) => {
                unimplemented!() // TODO
            }
        }
    }

    fn get_register(&self, register: Register) -> u8 {
        self.registers[register as usize]
    }

    fn set_register(&mut self, register: Register, value: u8) {
        self.registers[register as usize] = value;
    }

    fn set_vf_register(&mut self, value: u8) {
        self.registers[0xF] = value;
    }

    fn set_vf_confitional(&mut self, conditional: bool) {
        let value = if conditional { 1 } else { 0 };
        self.set_vf_register(value);
    }

    fn push_stack(&mut self) {
        self.stack[self.stack_pointer] = self.program_counter + 2;
        self.stack_pointer += 1;
    }

    fn pop_stack(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }

    fn get_memory(&self, index: u16) -> u8 {
        self.memory[index as usize]
    }

    fn set_memory(&mut self, index: u16, value: u8) {
        self.memory[index as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_and_reset() {
        let mut vm = Vm::new();
        let rom = vec![0xFF, 0xF1, 0x01, 0x22];
        vm.load(rom.clone());
        assert_eq!(vm.memory[MEMORY_START..MEMORY_START + 4], rom);

        vm.reset();
        for index in MEMORY_START..MEMORY_SIZE {
            assert_eq!(vm.get_memory(index as u16), 0);
        }
    }

    #[test]
    fn call_subroutine_jump_and_return() {
        let mut vm = Vm::new();
        vm.load(vec![
            0x22, 0x04, // 2204 - Call to addr 204
            0x12, 0x00, // 1200 - Jump to addr 200
            0x00, 0xEE, // 00EE - Return
        ]);

        vm.cycle(); // Call to addr 204
        assert_eq!(vm.stack[0], 0x202);
        assert_eq!(vm.stack_pointer, 1);
        assert_eq!(vm.program_counter, 0x204);

        vm.cycle();
        assert_eq!(vm.stack_pointer, 0);
        assert_eq!(vm.program_counter, 0x202);

        vm.cycle();
        assert_eq!(vm.program_counter, 0x200);
    }

    #[test]
    fn arathmatic_and_bit_operations() {
        let mut vm = Vm::new();
        // Registers labled as V[x]
        let program = vec![
            0x61, 0xF0, // v1 = 0xf0
            0x71, 0x11, // v1 = 0xf0 + 0x11
            0x82, 0x10, // v2 = v1
            0x61, 0xF0, // v1 = 0xf0
            0x62, 0x11, // v2 = 0x11
            0x81, 0x21, // v1 = v1 | v2 => 0xf1
            0x81, 0x22, // v1 = v1 & v2 => 0x11
            0x61, 0x21, // v1 = 0x21
            0x81, 0x23, // v1 = v1 ^ v2 => 0x30
            0x61, 0xF0, // v1 = 0xf0
            0x81, 0x24, // v1 = v1 + v2 => 0x01; vf = 0x01
            0x81, 0x25, // v1 = v1 - v2 => 0xf0; vf = 0x00
        ];

        vm.load(program);

        vm.cycle();
        assert_eq!(vm.get_register(1), 0xF0);

        vm.cycle();
        assert_eq!(vm.get_register(1), 0x01);
        assert_eq!(vm.get_register(0xf), 0x00);

        vm.cycle();
        assert_eq!(vm.get_register(1), vm.get_register(2));

        vm.cycle();
        vm.cycle();
        vm.cycle();
        assert_eq!(vm.get_register(1), 0xf1);

        vm.cycle();
        assert_eq!(vm.get_register(1), 0x11);

        vm.cycle();
        vm.cycle();
        assert_eq!(vm.get_register(1), 0x30);

        vm.cycle();
        vm.cycle();
        assert_eq!(vm.get_register(1), 0x01);
        assert_eq!(vm.get_register(0xf), 0x01);

        vm.cycle();
        assert_eq!(vm.get_register(1), 0xf0);
        assert_eq!(vm.get_register(0xf), 0x00);
    }

    #[test]
    fn set_i_register() {
        let mut vm = Vm::new();

        let program = vec![
            0xA5, 0x00, // I = 0x500
            0x60, 0x05, // V0 = 0x05
            0xF0, 0x1E, // I = I + V0
            0x60, 0x03, // V0 = 0x03
            0xF0, 0x29, // TODO fonts
            0xA5, 0x00, // I = 0x500
            0x60, 0xDA, // V0 = 0xDA
            0xF0, 0x33, // TODO store bcd
        ];

        vm.load(program);
        assert_eq!(vm.index, 0x0);

        vm.cycle();
        assert_eq!(vm.index, 0x500);

        vm.cycle();
        vm.cycle();
        assert_eq!(vm.index, 0x505);

        // TODO: More implementation
    }
}
