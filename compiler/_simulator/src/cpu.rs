use std::io;

#[derive(Debug, Clone, Copy)]
pub enum Instr {
    Invalid(u8),
    NoArg(u8),
    SingleArg(u8, u16),
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


    pub fn dump_status(&self) {
        println!("AC: {}, PC: {}, mem[0..16]: {:?}",
            self.ac, self.pc, &self.mem[0..16]);
    }


    pub fn cycle(&mut self) -> Option<Instr> {
        if !self.running {
            return None;
        }

        let opc = self.bump();
        // println!("#> {}", opc);
        let instr = match opc {
            // NOP
            0x00 => Instr::NoArg(opc),
            // QUT
            0x01 => {
                self.running = false;
                Instr::NoArg(opc)
            },

            // LDA
            0x10 => {
                let x = self.dbump();
                self.ac = self.mem_word(x);
                Instr::SingleArg(opc, x)
            }

            // ADD
            0x30 => {
                let x = self.dbump();
                self.ac += self.mem_word(x);
                Instr::SingleArg(opc, x)
            },

            // OUTA
            0xC0 => {
                let ac = self.ac;
                self.output(ac);
                Instr::NoArg(opc)
            },
            _    => {
                self.running = false;
                Instr::Invalid(opc)
            },
        };

        Some(instr)
    }

    fn output(&mut self, data: u16) {
        println!("> {}", data);
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
