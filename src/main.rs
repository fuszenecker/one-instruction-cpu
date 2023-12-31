use std::fs::File;
use std::io::{self, Read};

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
            let addr_a = parser(first) as u8;
            let addr_b = parser(token.next().unwrap()) as u8;
            let jmpaddr = parser(token.next().unwrap());
            prog.push((addr_a, addr_b, jmpaddr));
        }
    }
    (prog, rom)
}

// -- VCPU Runner --

struct Vcpu {
    data: [i16; 256],
}

impl Vcpu {
    // Memory & memory mapped functions
    fn mem_rd(&self, addr: u8) -> i16 {
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
            _ => self.data[addr as usize],
        }
    }

    // Memory & memory mapped functions
    fn mem_wr(&mut self, addr: u8, value: i16) {
        match addr {
            // Stdin
            0 => (),
            // Stdout
            1 => print!("{}", char::from_u32(value as u32).unwrap()),
            // ROM write not allowed
            0x80..=0xff => (),
            // RAM
            _ => self.data[addr as usize] = value,
        }
    }

    pub fn new() -> Self {
        let data: [i16; 256] = [0; 256];
        Vcpu { data }
    }

    pub fn runner(&mut self, prog: &[Instr], rom: &[i16]) {
        self.data[0x80..0x80 + rom.len()].copy_from_slice(rom);
        let mut pc = 0;

        // CPU run
        while pc < prog.len() {
            let instr = prog[pc];
            let result = self.mem_rd(instr.0) - self.mem_rd(instr.1);
            self.mem_wr(instr.0, result);
            if result <= 0 {
                pc += instr.2 as usize;
            }
            pc += 1;
        }
    }
}

fn main() {
    if let Some(fname) = std::env::args().nth(1) {
        let mut file = File::open(fname).expect("program file not found");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("failed to read");
        let (prog, rom) = compiler(&src);
        let mut vcpu = Vcpu::new();
        vcpu.runner(&prog, &rom);
    } else {
        eprintln!("usage: brainfuck <file.bf>");
    }
}
