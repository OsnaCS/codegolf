use std::io;

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

    pub fn show_mem(&self) {
        println!("MEM: {:?}", &self.mem[..16]);
    }

    pub fn cycle(&mut self) -> bool {
        if !self.running {
            return false;
        }

        match self.bump() {
            // NOP
            0x00 => {},
            // QUT
            0x01 => self.running = false,

            _    => {},
        };

        true
    }


    fn bump(&mut self) -> u8 {
        let out = self.mem[self.pc as usize];

        // FIXME: Overflow is wanted here. Tell rust to not check for
        // overflows. For now a simple workaround.
        if self.pc == 0xFFFF {
            self.pc = 0;
        } else {
            self.pc += 1;
        }
        out
    }
}
