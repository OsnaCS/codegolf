use std::io;
use std::fmt::{self, Display, Formatter};
use term_painter::{ToStyle, Color, Attr};

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Invalid(u8),
    NoArg(u8),
    SingleArg(u8, u16),
}

impl Instr {
    pub fn size(&self) -> usize {
        match *self {
            Instr::Invalid(_) | Instr::NoArg (_) => 1,
            Instr::SingleArg(_, _) => 3,
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Instr::Invalid(opc) =>
                write!(f, "Invalid ({})", opc),
            Instr::NoArg(opc) =>
                write!(f, "{:0<2x}", opc),
            Instr::SingleArg(opc, x) =>
                write!(f, "{:0<2x} {:0<4x}", opc, x),
        }
    }
}

pub struct Cpu {
    mem: Vec<u8>,
    ac: u16,
    pc: u16,
    running: bool,
}


impl Cpu {
    pub fn new() -> Cpu {
        let mut c = Cpu {
            mem: Vec::with_capacity(0x10000),
            ac: 0,
            pc: 0,
            running: true,
        };
        unsafe { c.mem.set_len(0x10000) };
        c
    }

    pub fn load_program<R: io::Read>(&mut self, mut prog: R) -> io::Result<()> {
        let mut count = 0;
        loop {
            let c = try!(prog.read(&mut self.mem[count..]));
            if c == 0 {
                break
            }
            count += c;
        }

        Ok(())
    }


    pub fn dump_regs(&self) {
        Color::Green.with(|| println!("AC: {}, PC: {}",
            Color::White.paint(self.ac),
            Color::White.paint(self.pc))
        );
    }

    pub fn dump_mem(&self, lo: u16, hi: u16) {
        Color::Green.with(|| print!("mem[{}..{}]: ",
            Color::White.paint(lo),
            Color::White.paint(hi)));
        for x in lo..hi {
            print!("{:0<2x} ", self.mem[x as usize]);
        }
        println!("");
    }

    pub fn next_instr(&mut self) -> Option<Instr> {
        if !self.running {
            None
        } else {
            let before_pc = self.pc;
            let opc = self.bump();
            let instr = match opc {
                // NOP | QUT
                0x00 | 0x01 => Instr::NoArg(opc),

                // LDA | STA | COM | NEG | ADD | AND
                0x10 | 0x20 | 0x30 | 0x31 | 0x38 | 0x39 | 0x40 =>
                    Instr::SingleArg(opc, self.dbump()),

                // JMPI JMP JZI JZI
                0x90 ... 0x93 => Instr::SingleArg(opc, self.dbump()),

                // OUTA
                0xC0 => Instr::NoArg(opc),

                _    => Instr::Invalid(opc),
            };
            self.pc = before_pc;
            Some(instr)
        }
    }

    pub fn cycle(&mut self, instr: Instr) {
        if !self.running {
            return;
        }

        self.pc += instr.size() as u16;
        match instr {
            Instr::NoArg(opc) => {
                match opc {
                    // NOP
                    0x00 => {},
                    // QUT
                    0x01 => { self.running = false; },
                    // OUTA
                    0xC0 => {
                        let ac = self.ac;
                        self.output(ac);
                    },

                    _ => panic!("Instruction not implemented!"),
                }

            },
            Instr::SingleArg(opc, x) => {
                match opc {
                    // LDA
                    0x10 => { self.ac = self.mem_word(x); },
                    // STA
                    0x20 => {},
                    // ADD
                    0x30 => { self.ac += self.mem_word(x); },
                    // SUB
                    0x31 => { self.ac -= self.mem_word(x); },
                    // COM
                    0x38 => { self.ac = !self.ac; }
                    // NEG
                    0x39 => { self.ac = !self.ac + 1; }
                    // AND
                    0x40 => { self.ac &= self.mem_word(x); },

                    // JMPI
                    0x90 => { self.pc = x; },
                    0x91 => { self.pc = self.mem_word(x); },
                    0x92 => { if self.ac == 0 { self.pc = x; } },
                    0x93 => { if self.ac == 0 { self.pc = self.mem_word(x); } },

                    _ => panic!("Instruction not implemented!"),
                }
            },
            Instr::Invalid(_) => { self.running = false; }
        };
    }

    fn output(&mut self, data: u16) {
        println!("> {}", Color::Yellow.paint(data));
    }

    fn mem_byte(&self, x: u16) -> u8 {
        self.mem[x as usize]
    }
    fn mem_word(&self, x: u16) -> u16 {
        (self.mem[x as usize] as u16) << 8 |
            self.mem[x as usize + 1] as u16
    }

    fn bump(&mut self) -> u8 {
        let out = self.mem_byte(self.pc);

        // FIXME: Overflow is wanted here. Tell rust to not check for
        // overflows. For now a simple workaround.
        if self.pc == 0xFFFF {
            self.pc = 0;
        } else {
            self.pc += 1;
        }
        out
    }

    fn dbump(&mut self) -> u16 {
        let out = (self.mem[self.pc as usize] as u16) << 8 |
            self.mem[self.pc as usize + 1] as u16;

        // FIXME: Overflow is wanted here. Tell rust to not check for
        // overflows. For now a simple workaround.
        if self.pc >= 0xFFFE {
            self.pc -= 0xFFFE;
        } else {
            self.pc += 2;
        }
        out
    }
}
