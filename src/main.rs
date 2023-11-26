use std::fs::File;
use std::io::{self, Read};

const DEBUG: bool = false;

// addrA addrB jmpaddr
type Instr = (u8, u8, i16);

fn parser(value: &str) -> i16 {
    if value.starts_with("0x") {
        i16::from_str_radix(value.strip_prefix("0x").unwrap(), 16).unwrap()
    } else if value.starts_with("-0x") {
        -i16::from_str_radix(value.strip_prefix("-0x").unwrap(), 16).unwrap()
    } else {
        value.parse().unwrap()
    }
}

fn compiler(src: &str) -> (Vec<Instr>, Vec<i16>) {
    let mut prog = vec![];
    let mut rom = vec![];
    for (_i, line) in src.lines().enumerate() {
        let rowstart = line.split('#').next().unwrap();
        if rowstart.len() < 5 {
            continue;
        }
        let mut token = rowstart.split_whitespace();
        let first = token.next().unwrap();
        if first == "rom" {
            for x in token {
                rom.push(parser(x));
            }
        } else {
            // eprintln!("{line}");
            let addr_a = parser(first) as u8;
            let addr_b = parser(token.next().unwrap()) as u8;
            let jmpaddr = parser(token.next().unwrap());
            prog.push((addr_a, addr_b, jmpaddr));
        }
    }
    if DEBUG {
        eprintln!("Prog: {prog:?}\nROM: {rom:?}");
    }
    (prog, rom)
}

// Memory & memory mapped functions
fn mem_rd(data: &[i16], addr: u8) -> i16 {
    match addr {
        // Stdin
        0 => {
            let mut inp: [u8; 1] = [0; 1];
            io::stdin().read_exact(&mut inp).expect("failed to read");
            inp[0] as i16
        }
        // Stdout
        1 => 0,
        // RAM, ROM
        _ => data[addr as usize],
    }
}

// Memory & memory mapped functions
fn mem_wr(data: &mut [i16], addr: u8, value: i16) {
    match addr {
        // Stdin
        0 => (),
        // Stdout
        1 => print!("{}", char::from_u32(value as u32).unwrap()),
        // RAM, ROM write not allowed
        _ => {
            if addr < 0x80 {
                data[addr as usize] = value
            }
        }
    }
}

fn vcpu_runner(prog: &[Instr], rom: &[i16]) {
    // RAM init with rom copy
    let mut data: [i16; 256] = [0; 256];
    data[0x80..0x80 + rom.len()].copy_from_slice(rom);
    let mut pc = 0;

    // CPU run
    while pc < prog.len() {
        let instr = prog[pc];
        if DEBUG {
            eprintln!("{pc}. {instr:?}");
        }
        let result = mem_rd(&data, instr.0) - mem_rd(&data, instr.1);
        mem_wr(&mut data, instr.0, result);
        // Jump_rel if zero or less
        if result <= 0 {
            pc += instr.2 as usize;
        }
        pc += 1;
    }
}

fn main() {
    if let Some(fname) = std::env::args().nth(1) {
        let mut file = File::open(fname).expect("program file not found");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("failed to read");
        let (prog, rom) = compiler(&src);
        vcpu_runner(&prog, &rom);
    } else {
        eprintln!("usage: brainfuck <file.bf>");
    }
}
