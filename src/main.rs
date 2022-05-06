const OP_HALT: u8 = 0x0;
const OP_RET : u8 = 0x1;
const OP_SHL : u8 = 0x2;
const OP_SHR : u8 = 0x3;
const OP_ROL : u8 = 0x4;
const OP_ROR : u8 = 0x5;
const OP_NOT : u8 = 0x6;
const OP_AND : u8 = 0x7;
const OP_OR  : u8 = 0x8;
const OP_XOR : u8 = 0x9;
const OP_JUMP: u8 = 0xA;
const OP_CALL: u8 = 0xB;
const OP_LOAD: u8 = 0xC;
const OP_STOR: u8 = 0xD;
const OP_BREQ: u8 = 0xE;
const OP_BRNE: u8 = 0xF;

#[derive(Debug)]
struct Registers {
    ip: u16,
    rp: u16,
    r0: u8,
    r1: u8,
    r2: u8,
    r3: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            ip: 0,
            rp: 0,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
        }
    }

    pub fn get(&self, r: u8) -> u8 {
        match r {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            _ => panic!("No such register"),
        }
    }

    pub fn set(&mut self, r: u8, value: u8) {
        match r {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            _ => panic!("No such register"),
        }
    }
}

struct Core {
    memory: [u8; Self::MEM_SIZE],
    regs: Registers,
    is_halted: bool,
}

impl Core {
    const MEM_SIZE: usize = 0x10000;
    pub fn new() -> Self {
        Self {
            memory: [0; Self::MEM_SIZE],
            regs: Registers::new(),
            is_halted: false,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.memory[..data.len()].copy_from_slice(data);
    }

    pub fn step(&mut self) {
        let instr = self.next_byte();
        let opcode = (instr >> 4) & 0xF;

        let r_a = (instr >> 2) & 0x3;
        let r_b = instr & 0x3;
        let addr = if opcode >= OP_JUMP { self.next_short() } else { 0 };

        println!("OP={} A={} B={}", opcode, r_a, r_b);

        match opcode {
            OP_HALT => self.is_halted = true, //HALT
            OP_RET => self.regs.ip = self.regs.rp,
            OP_SHL => { // SHL
                let value = self.regs.get(r_a);
                self.regs.set(r_a, value.wrapping_shl(1));
            },
            OP_SHR => { // SHR
                let value = self.regs.get(r_a);
                self.regs.set(r_a, value.wrapping_shr(1));
            },
            OP_ROL => { // ROL
                let value = self.regs.get(r_a);
                self.regs.set(r_a, value.rotate_left(1));
            },
            OP_ROR => { // ROL
                let value = self.regs.get(r_a);
                self.regs.set(r_a, value.rotate_right(1));
            },
            OP_NOT => {
                let value = self.regs.get(r_a);
                self.regs.set(r_a, !value);
            },
            OP_AND => {
                let val_a = self.regs.get(r_a);
                let val_b = self.regs.get(r_b);
                self.regs.set(r_a, val_a & val_b);
            },
            OP_OR => {
                let val_a = self.regs.get(r_a);
                let val_b = self.regs.get(r_b);
                self.regs.set(r_a, val_a | val_b);
            },
            OP_XOR => {
                let val_a = self.regs.get(r_a);
                let val_b = self.regs.get(r_b);
                self.regs.set(r_a, val_a ^ val_b);
            },
            OP_JUMP => {
                self.regs.ip = addr;
            },
            OP_CALL => {
                self.regs.rp = self.regs.ip;
                self.regs.ip = addr;
            },
            OP_LOAD => {
                let value = self.memory[addr as usize];
                self.regs.set(r_a, value);
            },
            OP_STOR => {
                let value = self.regs.get(r_a);
                self.memory[addr as usize] = value;
            },
            OP_BREQ => {
                let val_a = self.regs.get(r_a);
                let val_b = self.regs.get(r_b);
                if val_a == val_b {
                    self.regs.ip = addr;
                }
            },
            OP_BRNE => {
                let val_a = self.regs.get(r_a);
                let val_b = self.regs.get(r_b);
                if val_a != val_b {
                    self.regs.ip = addr;
                }
            },
            opcode => panic!("Unknown opcode {}", opcode),
        }

        println!("REGS: {:x?}", self.regs);
    }
    
    fn next_byte(&mut self) -> u8 {
        let value = self.memory[self.regs.ip as usize];
        self.regs.ip = self.regs.ip.wrapping_add(1);
        value
    }

    fn next_short(&mut self) -> u16 {
        let a = self.next_byte();
        let b = self.next_byte();
        u16::from_be_bytes([a,b])
    }
}

fn main() {

    let mut core = Core::new();
    {
        use std::env;
        use std::fs::File;
        use std::io::prelude::*;

        let filename = env::args().nth(1).expect("No filename given");
        println!("Loading {}", filename);

        let mut file = File::open(filename).expect("File does not exist");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Could not read file");

        core.load(&buffer);
    }

    while !core.is_halted {
        core.step();
    }
}
