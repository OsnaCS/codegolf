//! Debugger
//! ========
//! The debugger uses the CPU to execute a program. It can be executed
//! in debug or release mode. The debug mode offers interactive
//! control via command line with many features such as register and
//! memory dump and breakpoints.

use cpu::{Cpu, Instr};
use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RunMode {
    Debug,
    Release
}

#[derive(Debug)]
enum RunErr {
    Io(io::Error),  // IO error occured
    MaxCycle(usize),    // Number of cycle limit was reached
    InvalidInstr(u8)    // Invalid instruction
}

#[derive(Debug)]
enum RunOk {
    User,   // user stopped execution via debugger
    QuitInstr,  // a QUT instruction was executed
}

pub type RunResult = Result<(RunOk, usize), RunErr>;

pub struct Debugger {
    cpu: Cpu,
    run_mode: RunMode,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            cpu: Cpu::new(),
            run_mode: RunMode::Release,
        }
    }

    pub fn set_run_mode(&mut self, mode: RunMode) {
        self.run_mode = mode;
    }

    pub fn run<R: io::Read>(&mut self, prog: R) -> RunResult {
        match self.cpu.load_program(prog) {
            Err(e) => return Err(RunErr::Io(e)),
            _ => self.log_debug("Loaded program into CPU"),
        }


        let mut count = 0;
        loop {
            self.cpu.dump_status();
            let instr = self.cpu.cycle();
            println!("##> {:?}", instr);
            count += 1;

            // break loop
            match instr {
                None => return Ok((RunOk::QuitInstr, count)),
                Some(Instr::Invalid(opc)) => {
                    return Err(RunErr::InvalidInstr(opc))
                },
                _ => {},
            }
            if count > 20 {
                return Err(RunErr::MaxCycle(20));
            }

            // self.cpu.show_mem();
        }
    }

    pub fn log_debug(&self, m: &str) {
        if self.run_mode == RunMode::Debug {
            println!("~> DEBUG: {}", m);
        }
    }
}
