//! Debugger
//! ========
//! The debugger uses the CPU to execute a program. It can be executed
//! in debug or release mode. The debug mode offers interactive
//! control via command line with many features such as register and
//! memory dump and breakpoints.

use cpu::Cpu;
use std::io;

pub enum RunMode {
    Debug,
    Release
}

#[derive(Debug)]
enum RunErr {
    Io(io::Error),  // IO error occured
    MaxCycle(usize),    // Number of cycle limit was reached
}

#[derive(Debug)]
enum RunOk {
    User,   // user stopped execution via debugger
    QuitInstr,  // a QUT instruction was executed
}

pub type RunResult = Result<RunOk, RunErr>;

pub struct Debugger {
    cpu: Cpu,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            cpu: Cpu::new(),
        }
    }

    pub fn run<R: io::Read>(&mut self, prog: R, mode: RunMode) -> RunResult {
        match self.cpu.load_program(prog) {
            Err(e) => return Err(RunErr::Io(e)),
            _ => self.log_debug("Loaded program into CPU"),
        }


        let mut count = 0;
        loop {
            let quit = self.cpu.cycle();
            count += 1;

            // break loop
            if quit {
                return Ok(RunOk::QuitInstr);
            }
            if count > 20 {
                return Err(RunErr::MaxCycle(20));
            }


            self.cpu.show_mem();
        }
    }

    fn log_debug(&self, m: &str) {
        println!("~> DEBUG: {}", m);
    }
}
