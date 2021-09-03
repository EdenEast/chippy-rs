use crate::{
    emu::display::Display,
    emu::font::FONT_SET,
    emu::instruction::{Instruction, RegisterValuePair, TargetSourcePair},
};
use byteorder::{BigEndian, ReadBytesExt};

use super::input::Input;

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
    pub display: Display,
    pub input: Input,
    memory: [u8; MEMORY_SIZE],
    registers: [Register; REGISTER_SIZE],
    stack: [StackEntry; STACK_SIZE],
    stack_pointer: usize,
    index: u16,
    program_counter: u16,
    deplay_timer: u8,
    sound_timer: u8,
    wait_for_key: Option<u8>,
    should_draw: bool,
}

impl Vm {
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        for (index, character) in FONT_SET.iter().enumerate() {
            memory[index] = *character;
        }

        Self {
            display: Display::new(),
            input: Input::new(),
            memory,
            registers: [0; REGISTER_SIZE],
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            index: 0,
            program_counter: INITIAL_PROGRAM_COUNTER,
            deplay_timer: 0,
            sound_timer: 0,
            wait_for_key: None,
            should_draw: false,
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

        self.display.clear();
        self.registers = [0; REGISTER_SIZE];
        self.stack = [0; STACK_SIZE];
        self.stack_pointer = 0;
        self.index = 0;
        self.program_counter = INITIAL_PROGRAM_COUNTER;
        self.should_draw = false;
    }

    pub fn cycle(&mut self) {
        if self.should_draw {
            self.should_draw = false;
        }

        let position = self.program_counter as usize;
        let mut parts = &self.memory[position..position + 2];
        let opcode = parts.read_u16::<BigEndian>().unwrap();

        self.program_counter = match self.execute_instruction(opcode) {
            ProgramCounter::Next => self.program_counter + 2,
            ProgramCounter::Skip => self.program_counter + 4,
            ProgramCounter::Jump(addr) => addr,
        };
    }

    pub fn execute_instruction(&mut self, opcode: u16) -> ProgramCounter {
        match Instruction::parse(opcode) {
            Instruction::CallMachineCode(_) => {
                ProgramCounter::Next // TODO
            }
            Instruction::ClearDisplay => {
                ProgramCounter::Next // TODO
            }
            Instruction::Return => ProgramCounter::Jump(self.pop_stack()),
            Instruction::Jump(addr) => ProgramCounter::Jump(addr),
            Instruction::Call(addr) => {
                self.push_stack();
                ProgramCounter::Jump(addr)
            }
            Instruction::SkipIfEq(RegisterValuePair { register, value }) => {
                skip_if(self.get_register(register) == value)
            }
            Instruction::SkipIfNeq(RegisterValuePair { register, value }) => {
                skip_if(self.get_register(register) != value)
            }
            Instruction::SkipIfRegEq(TargetSourcePair { target, source }) => {
                skip_if(self.get_register(target) == self.get_register(source))
            }
            Instruction::SetReg(RegisterValuePair { register, value }) => {
                self.set_register(register, value);
                ProgramCounter::Next
            }
            Instruction::AddValueToReg(RegisterValuePair { register, value }) => {
                let (sum, _) = self.get_register(register).overflowing_add(value);
                self.set_register(register, sum);
                ProgramCounter::Next
            }
            Instruction::SetRegXToRegY(TargetSourcePair { target, source }) => {
                self.set_register(target, self.get_register(source));
                ProgramCounter::Next
            }
            Instruction::BitXOrY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) | self.get_register(source);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Instruction::BitXAndY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) & self.get_register(source);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Instruction::BitXXorY(TargetSourcePair { target, source }) => {
                let result = self.get_register(target) ^ self.get_register(source);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Instruction::AddYToX(TargetSourcePair { target, source }) => {
                let (result, did_overflow) = self
                    .get_register(target)
                    .overflowing_add(self.get_register(source));
                self.set_vf_confitional(did_overflow);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Instruction::SubYFromX(TargetSourcePair { target, source }) => {
                let (result, did_overflow) = self
                    .get_register(target)
                    .overflowing_sub(self.get_register(source));
                self.set_vf_confitional(!did_overflow);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Instruction::ShiftRight(TargetSourcePair { target, source }) => {
                let value = self.get_register(target);
                self.set_vf_register(value & 0xF);
                self.set_register(target, value >> 1);
                ProgramCounter::Next
            }
            Instruction::SubXFromYIntoX(TargetSourcePair { target, source }) => {
                let (result, did_overflow) = self
                    .get_register(source)
                    .overflowing_sub(self.get_register(target));
                self.set_vf_confitional(!did_overflow);
                self.set_register(target, result);
                ProgramCounter::Next
            }
            Instruction::ShiftLeft(TargetSourcePair { target, source }) => {
                let value = self.get_register(target);
                self.set_vf_register(value >> 7);
                self.set_register(target, value << 1);
                ProgramCounter::Next
            }
            Instruction::SkipIfDifferent(TargetSourcePair { target, source }) => {
                skip_if(self.get_register(target) != self.get_register(source))
            }
            Instruction::SetI(value) => {
                self.index = value;
                ProgramCounter::Next
            }
            Instruction::JumpNPlusPC(addr) => {
                ProgramCounter::Jump(addr + self.get_register(0x0) as u16)
            }
            Instruction::Random(RegisterValuePair { register, value }) => {
                // TODO: get random number between 0, 255
                let random = 0x5d;
                self.set_register(register, random & value);
                ProgramCounter::Next
            }
            Instruction::Draw { x, y, n } => {
                let new_vf = self.display.draw(
                    self.get_register(x) as usize,
                    self.get_register(y) as usize,
                    &self.memory[self.index as usize..(self.index + n as u16) as usize],
                );
                self.set_vf_register(new_vf);
                self.should_draw = true;
                ProgramCounter::Next
            }
            Instruction::SkipIfKeyPressed(register) => {
                let value = self.get_register(register);
                skip_if(self.input.is_pressed(value))
            }
            Instruction::SkipIfNotKeyPressed(register) => {
                let value = self.get_register(register);
                skip_if(!self.input.is_pressed(value))
            }
            Instruction::SetXAsDT(register) => {
                self.set_register(self.get_register(register), self.deplay_timer);
                ProgramCounter::Next
            }
            Instruction::WaitInputStoreIn(register) => {
                self.wait_for_key = Some(self.get_register(register));
                ProgramCounter::Next
            }
            Instruction::SetDTAsX(register) => {
                self.deplay_timer = self.get_register(register);
                ProgramCounter::Next
            }
            Instruction::SetSTAsX(register) => {
                self.sound_timer = self.get_register(register);
                ProgramCounter::Next
            }
            Instruction::AddXToI(register) => {
                let (result, _) = self
                    .index
                    .overflowing_add(self.get_register(register) as u16);
                self.index = result;
                ProgramCounter::Next
            }
            Instruction::SetIToFontSprite(register) => {
                self.index = self.get_register(register) as u16 * 5; // sprites are 5 bytes long
                ProgramCounter::Next
            }
            Instruction::StoreBCD(register) => {
                let value = self.get_register(register);
                self.set_memory(self.index, value / 100); // hundreds
                self.set_memory(self.index + 1, (value % 100) / 10); // tens
                self.set_memory(self.index + 2, value % 10); // ones
                ProgramCounter::Next
            }
            Instruction::DumpRegisters(limit) => {
                for r in 0..=limit {
                    self.set_memory(self.index, self.get_register(r));
                    self.index += 1;
                }
                ProgramCounter::Next
            }
            Instruction::LoadRegisters(limit) => {
                for r in 0..=limit {
                    self.set_register(r, self.get_memory(self.index));
                    self.index += 1;
                }
                ProgramCounter::Next
            }
            Instruction::Invalid(_) => ProgramCounter::Next, // Skip invalid instructions
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

    fn cycle(vm: &mut Vm, n: usize) {
        for _ in 0..n {
            vm.cycle()
        }
    }

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
    fn instructions_with_i_register() {
        let mut vm = Vm::new();

        let program = vec![
            0xA5, 0x00, // ld i, 0x500
            0x60, 0x05, // ld v0, 0x05
            0xF0, 0x1E, // add i, v0
            0x60, 0x03, // ld v0, 0x03
            0xF0, 0x29, // ld f, v0
            0xA5, 0x00, // ld i, 0x500
            0x60, 0xDA, // ld v0, 0xDA
            0xF0, 0x33, // ld b, v0
        ];

        vm.load(program);
        assert_eq!(vm.index, 0x0);

        vm.cycle();
        assert_eq!(vm.index, 0x500);

        vm.cycle();
        vm.cycle();
        assert_eq!(vm.index, 0x505);

        vm.cycle();
        vm.cycle();
        assert_eq!(vm.index, 0xF);

        vm.cycle();
        vm.cycle();
        vm.cycle();
        assert_eq!(vm.get_memory(vm.index), 2);
        assert_eq!(vm.get_memory(vm.index + 1), 1);
        assert_eq!(vm.get_memory(vm.index + 2), 8);
    }

    #[test]
    fn dump_and_load_registers() {
        let mut vm = Vm::new();
        let program = vec![
            0xA4, 0x00, // ld i, 0x400
            0x60, 0xF0, // ld v0, 0xF0
            0x61, 0xDD, // ld v1, 0xDD
            0x62, 0x1E, // ld v2, 0x1E
            0x63, 0x17, // ld v3, 0x17
            0x64, 0x4D, // ld v4, 0x4D
            0x65, 0x29, // ld v5, 0x29
            0xF5, 0x55, // ld [i], v5
            0x60, 0x00, // ld v0, 0x00
            0x61, 0x00, // ld v1, 0x00
            0x62, 0x00, // ld v2, 0x00
            0x63, 0x00, // ld v3, 0x00
            0x64, 0x00, // ld v4, 0x00
            0x65, 0x00, // ld v5, 0x00
            0xA4, 0x00, // ld i, 0x400
            0xF5, 0x65, // ld v5, [i]
        ];

        let register_values = vec![0xF0u8, 0xDDu8, 0x1Eu8, 0x17u8, 0x4Du8, 0x29u8];

        vm.load(program);

        // Load the index with value 0x400
        vm.cycle();
        assert_eq!(vm.index, 0x400);

        // Load registers V0 to V5
        cycle(&mut vm, 6);
        for (i, value) in register_values.iter().enumerate() {
            assert_eq!(vm.get_register(i as u8), *value);
        }

        // Execute the dump instruction for registers v0 - v5
        vm.cycle();
        assert_eq!(vm.index, 0x406);
        for i in 0..=5 {
            assert_eq!(vm.get_register(i), vm.get_memory(0x400 + i as u16))
        }

        // Clear registers v0 - v5 and reset I to 0x400
        cycle(&mut vm, 7);
        assert_eq!(vm.index, 0x400);
        for i in 0..=5 {
            assert_eq!(vm.get_register(i), 0x0);
        }

        // Execute the load instruction
        vm.cycle();
        for (i, value) in register_values.iter().enumerate() {
            assert_eq!(vm.get_register(i as u8), *value);
        }
    }

    // TODO: timers, input and control flow
}
