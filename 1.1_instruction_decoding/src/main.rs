use std::fs::File;
use std::io::{self, Read};
use std::fmt::Display;

#[macro_use]
extern crate enum_display_derive;

#[derive(Debug, PartialEq, Display)]
enum Instruction {
    MOV,
    UNKNOWN,
}

#[derive(Debug, PartialEq, Clone, Display)]
enum Register {
    AL,
    BL,
    CL,
    DL,
    AH,
    BH,
    CH,
    DH,
    AX,
    BX,
    CX,
    DX,
    SP,
    BP,
    SI,
    DI,
}

const REGISTERS: [[Register; 2]; 8] = [
    [Register::AL, Register::AX], // 000
    [Register::CL, Register::CX], // 001
    [Register::DL, Register::DX], // 010
    [Register::BL, Register::BX], // 011
    [Register::AH, Register::SP], // 100
    [Register::CH, Register::BP], // 101
    [Register::DH, Register::SI], // 110
    [Register::BH, Register::DI], // 111
];

fn main() -> io::Result<()> {
    let mut file = File::open("listing_0038_many_register_mov")?;
    let mut buffer = [0; 2];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read < 2 {
            break;
        }
        let (instruction, destination, source) = decode_bytes(&buffer);
        print!("\n{} {}, {}", instruction, destination, source);
    }

    Ok(())
}

// decode instruction from first byte
fn decode_instruction(byte: u8) -> &'static Instruction {
    let high_six_bits = byte >> 2;
    match high_six_bits {
        0b100010 => &Instruction::MOV,
        _ => &Instruction::UNKNOWN,
    }
}

// decode d, w flags from first byte
fn decode_d_w(byte: u8) -> (bool, usize) {
    let d = (byte & 0b00000000) == 0b00000010;
    let w = (byte & 0b00000001) as usize;
    (d, w)
}

// decode register byte
fn decode_registers(byte: u8, wide: usize) -> (&'static Register, &'static Register) {
    let registers = byte;
    let source_register = ((registers & 0b00111000) >> 3) as usize;
    let destination_register = (registers & 0b00000111) as usize;

    (&REGISTERS[destination_register][wide], &REGISTERS[source_register][wide])
}

// take two bytes and return a string of the instruction and registers
fn decode_bytes(buffer: &[u8; 2]) -> (&'static Instruction, &'static Register, &'static Register) {
    let instruction_name = decode_instruction(buffer[0]);
    let (_direction_to, wide) = decode_d_w(buffer[0]);
    let registers = decode_registers(buffer[1], wide);
    (instruction_name, registers.0, registers.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_instruction() {
        let buffer = include_bytes!("../listing_0037_single_register_mov");

        let instr = decode_bytes(buffer);

        assert_eq!(instr, (&Instruction::MOV, &Register::CX, &Register::BX));
    }

    #[test]
    fn test_multiple_instructions() {
        let buffer = include_bytes!("../listing_0038_many_register_mov");

        let expected = vec![
            (&Instruction::MOV, &Register::CX, &Register::BX),
            (&Instruction::MOV, &Register::CH, &Register::AH),
            (&Instruction::MOV, &Register::DX, &Register::BX),
            (&Instruction::MOV, &Register::SI, &Register::BX),
            (&Instruction::MOV, &Register::BX, &Register::DI),
            (&Instruction::MOV, &Register::AL, &Register::CL),
            (&Instruction::MOV, &Register::CH, &Register::CH),
            (&Instruction::MOV, &Register::BX, &Register::AX),
            (&Instruction::MOV, &Register::BX, &Register::SI),
            (&Instruction::MOV, &Register::SP, &Register::DI),
            (&Instruction::MOV, &Register::BP, &Register::AX),
        ];

        let mut count = 0;
        for word in buffer.chunks(2) {
            let instr = decode_bytes(&[word[0], word[1]]);
            assert_eq!(instr, expected[count]);
            count += 1;
        }
    }
}


