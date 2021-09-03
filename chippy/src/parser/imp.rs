use super::error::{LineError, ParseError, ParseResult};
use crate::emu::instruction::{Instruction, RegisterValuePair, TargetSourcePair};
use std::str::FromStr;

trait FromStrRadix: Sized {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, LineError>;
}

macro_rules! impl_str_radix {
    ($t: ty) => {
        impl FromStrRadix for $t {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, LineError> {
                <$t>::from_str_radix(src, radix).map_err(LineError::from)
            }
        }
    };
}
impl_str_radix!(u8);
impl_str_radix!(u16);

fn ts(target: u8, source: u8) -> TargetSourcePair {
    TargetSourcePair { target, source }
}

fn rv(register: u8, value: u8) -> RegisterValuePair {
    RegisterValuePair { register, value }
}

pub fn parse(program: &str) -> ParseResult<Vec<Instruction>> {
    let src = program.trim();
    let lines: Vec<(usize, &str)> = src.split('\n').enumerate().collect();

    lines
        .iter()
        .filter_map(|(ln, line)| {
            let trim = line.trim();
            if !trim.is_empty() {
                Some(parse_instr(line).map_err(|err| ParseError::Line(*ln, err)))
            } else {
                None
            }
        })
        .collect::<ParseResult<Vec<Instruction>>>()
}

fn parse_instr(line: &str) -> Result<Instruction, LineError> {
    use Instruction::*;
    let lo = line.to_lowercase();

    let first_space = lo.find(' ');
    let (instruction, tokens) = if let Some(pos) = first_space {
        let (instruction, rest) = lo.split_at(pos);
        let tokens = rest.split(',').map(|token| token.trim()).collect();
        (instruction, tokens)
    } else {
        (lo.as_str(), Vec::new())
    };

    match instruction {
        "sys" => Ok(CallMachineCode(parse_addr(tokens[0])?)),
        "cls" => Ok(ClearDisplay),
        "ret" => Ok(Return),
        "call" => Ok(Call(parse_addr(tokens[0])?)),
        "raw" => Ok(Invalid(parse_addr(tokens[0])?)),
        "skp" => Ok(SkipIfKeyPressed(parse_register(tokens[0])?)),
        "sknp" => Ok(SkipIfNotKeyPressed(parse_register(tokens[0])?)),
        "and" => Ok(BitXAndY(TargetSourcePair {
            target: parse_register(tokens[0])?,
            source: parse_register(tokens[1])?,
        })),
        "or" => Ok(BitXOrY(TargetSourcePair {
            target: parse_register(tokens[0])?,
            source: parse_register(tokens[1])?,
        })),
        "xor" => Ok(BitXXorY(TargetSourcePair {
            target: parse_register(tokens[0])?,
            source: parse_register(tokens[1])?,
        })),
        "rnd" => Ok(Random(RegisterValuePair {
            register: parse_register(tokens[0])?,
            value: parse_number(tokens[1])?,
        })),
        "shl" => {
            let source = match tokens.get(1) {
                Some(r) => parse_register(r)?,
                None => 0u8,
            };
            Ok(ShiftLeft(TargetSourcePair {
                target: parse_register(tokens[0])?,
                source,
            }))
        }
        "shr" => {
            let source = match tokens.get(1) {
                Some(r) => parse_register(r)?,
                None => 0u8,
            };
            Ok(ShiftRight(TargetSourcePair {
                target: parse_register(tokens[0])?,
                source,
            }))
        }
        "drw" => Ok(Draw {
            x: parse_register(tokens[0])?,
            y: parse_register(tokens[1])?,
            n: parse_number(tokens[2])?,
        }),
        "add" => match tokens[0] {
            "i" => Ok(AddXToI(parse_register(tokens[1])?)),
            _ => match tokens[1].chars().next() {
                Some('v') => Ok(AddYToX(TargetSourcePair {
                    target: parse_register(tokens[0])?,
                    source: parse_register(tokens[1])?,
                })),
                _ => Ok(AddValueToReg(RegisterValuePair {
                    register: parse_register(tokens[0])?,
                    value: parse_number(tokens[1])?,
                })),
            },
        },
        "sub" => Ok(SubYFromX(TargetSourcePair {
            target: parse_register(tokens[0])?,
            source: parse_register(tokens[1])?,
        })),
        "subn" => Ok(SubXFromYIntoX(TargetSourcePair {
            target: parse_register(tokens[0])?,
            source: parse_register(tokens[1])?,
        })),
        "se" => match tokens[1].chars().next() {
            Some('v') => Ok(SkipIfRegEq(TargetSourcePair {
                target: parse_register(tokens[0])?,
                source: parse_register(tokens[1])?,
            })),
            _ => Ok(SkipIfEq(RegisterValuePair {
                register: parse_register(tokens[0])?,
                value: parse_number(tokens[1])?,
            })),
        },
        "sne" => match tokens[1].chars().next() {
            Some('v') => Ok(SkipIfDifferent(TargetSourcePair {
                target: parse_register(tokens[0])?,
                source: parse_register(tokens[1])?,
            })),
            _ => Ok(SkipIfNeq(RegisterValuePair {
                register: parse_register(tokens[0])?,
                value: parse_number(tokens[1])?,
            })),
        },
        "ld" => match tokens[0] {
            "[i]" => Ok(DumpRegisters(parse_register(tokens[1])?)),
            "b" => Ok(StoreBCD(parse_register(tokens[1])?)),
            "dt" => Ok(SetDTAsX(parse_register(tokens[1])?)),
            "st" => Ok(SetSTAsX(parse_register(tokens[1])?)),
            "f" => Ok(SetIToFontSprite(parse_register(tokens[1])?)),
            "i" => Ok(SetI(parse_addr(tokens[1])?)),
            _ => match tokens[1] {
                "k" => Ok(WaitInputStoreIn(parse_register(tokens[0])?)),
                "dt" => Ok(SetXAsDT(parse_register(tokens[0])?)),
                "[i]" => Ok(LoadRegisters(parse_register(tokens[0])?)),
                _ => match tokens[1].chars().next() {
                    Some('v') => Ok(SetRegXToRegY(TargetSourcePair {
                        target: parse_register(tokens[0])?,
                        source: parse_register(tokens[1])?,
                    })),
                    _ => Ok(SetReg(RegisterValuePair {
                        register: parse_register(tokens[0])?,
                        value: parse_number(tokens[1])?,
                    })),
                },
            },
        },
        "jp" => match tokens.len() {
            1 => Ok(Jump(parse_addr(tokens[0])?)),
            2 => {
                if tokens[0] != "v0" {
                    Err(LineError::WrongJumpRegister)
                } else {
                    Ok(JumpNPlusPC(parse_addr(tokens[1])?))
                }
            }
            _ => Err(LineError::WrongNumberOfArguments(1, tokens.len())),
        },
        _ => Err(LineError::InvalidInstruction(instruction.to_string())),
    }
}

fn parse_number<T>(number: &str) -> Result<T, LineError>
where
    T: FromStrRadix + FromStr<Err = std::num::ParseIntError>,
{
    match number.strip_prefix("0x") {
        Some(slice) => T::from_str_radix(slice, 16),
        None => number.parse::<T>().map_err(LineError::from),
    }
}

fn parse_register(token: &str) -> Result<u8, LineError> {
    match token.chars().next() {
        Some('v') => match token.len() {
            2 => u8::from_str_radix(&token[1..], 16)
                .map_err(|err| LineError::InvalidRegister(token.to_string())),
            _ => Err(LineError::InvalidRegister(token.to_string())),
        },
        _ => Err(LineError::InvalidRegister(token.to_string())),
    }
}

fn parse_addr(token: &str) -> Result<u16, LineError> {
    let slice = token.strip_prefix("0x").unwrap_or(token);
    u16::from_str_radix(slice, 16).map_err(LineError::from)
}
