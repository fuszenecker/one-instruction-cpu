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
                rom.push(parser(x) as i16);
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

fn vcpu_runner(prog: &[Instr], rom: &[i16]) {
    // RAM init with rom copy
    let mut data: [i16; 256] = [0; 256];
    data[0x80..0x80 + rom.len()].copy_from_slice(rom);
    let mut pc = 0;
    while pc < prog.len() {
        let instr = prog[pc];
        if DEBUG {
            eprintln!("{pc}. {instr:?}");
        }
        // RAM 0. address --> I/O
        let reg_b = if instr.1 != 0 {
            data[instr.1 as usize]
        } else {
            let mut inp: [u8; 1] = [0; 1];
            io::stdin().read_exact(&mut inp).expect("failed to read");
            inp[0] as i16
        };
        // Substraction
        let reg_a = data[(instr.0 & 0x7f) as usize];
        let result = reg_a - reg_b;
        data[(instr.0 & 0x7f) as usize] = result;
        // Addr 0 --> RAM & out
        if instr.0 as usize == 0 {
            print!("{}", char::from_u32(data[instr.0 as usize] as u32).unwrap());
            data[instr.0 as usize] = 0; // zero
        }
        // Jump_rel if zero or lesser
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
