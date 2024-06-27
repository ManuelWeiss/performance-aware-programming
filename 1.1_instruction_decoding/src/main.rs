use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("listing_0038_many_register_mov")?;
    let mut buffer = [0; 2];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read < 2 {
            break;
        }

        // print!("\n{:08b} {:08b} ", buffer[0], buffer[1]);
        print!("\n{}", decode_bytes(buffer));
    }

    Ok(())
}

// map 3 bit binary to register name
fn register_name(register: u8, wide: bool) -> &'static str {
    if wide {
        match register {
            0b000 => "ax",
            0b001 => "cx",
            0b010 => "dx",
            0b011 => "bx",
            0b100 => "sp",
            0b101 => "bp",
            0b110 => "si",
            0b111 => "di",
            _ => "Unknown",
        }
    } else {
        match register {
            0b000 => "al",
            0b001 => "cl",
            0b010 => "dl",
            0b011 => "bl",
            0b100 => "ah",
            0b101 => "ch",
            0b110 => "dh",
            0b111 => "bh",
            _ => "Unknown",
        }
    }
}

// decode instruction from first byte
fn decode_instruction(byte: u8) -> String {
    let high_six_bits = byte >> 2;
    match high_six_bits {
        0b100010 => "mov",
        _ => "Unknown",
    }.to_string()
}

// decode d, w flags from first byte
fn decode_d_w(byte: u8) -> (bool, bool) {
    let d = (byte & 0b00000000) == 0b00000010;
    let w = (byte & 0b00000001) == 0b00000001;
    (d, w)
}

// decode register byte
fn decode_registers(byte: u8, wide: bool) -> String {
    let registers = byte;
    let source_register = (registers & 0b00111000) >> 3;
    let destination_register = registers & 0b00000111;

    // format!("\nsource {:03b}, destination {:03b}", source_register, destination_register);
    format!("{}, {}", register_name(destination_register, wide), register_name(source_register, wide))
}

// take two bytes and return a string of the instruction and registers
fn decode_bytes(buffer: [u8; 2]) -> String {
    let instruction_name = decode_instruction(buffer[0]);
    let (_direction_to, wide) = decode_d_w(buffer[0]);
    let registers = decode_registers(buffer[1], wide);
    format!("{} {}", instruction_name, registers)
}
