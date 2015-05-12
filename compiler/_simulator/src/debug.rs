//! Debugger
//! ========
//! The debugger uses the CPU to execute a program. It can be executed
//! in debug or release mode. The debug mode offers interactive
//! control via command line with many features such as register and
//! memory dump and breakpoints.

use cpu::{Cpu, Instr};
use term_painter::{ToStyle, Color, Attr};
use std::io;
use std::io::prelude::*;

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

const MAX_CYCLE_COUNT : usize = 200;

pub struct Debugger {
    cpu: Cpu,
    run_mode: RunMode,

    dump_regs: bool,
    dump_mem: bool,
    dump_instr: bool,

    paused: bool,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            cpu: Cpu::new(),
            run_mode: RunMode::Release,
            dump_regs: true,
            dump_mem: true,
            dump_instr: true,
            paused: true,
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
            // wait for user input
            if self.paused && self.run_mode == RunMode::Debug {
                self.parse_commands();
            }

            let next_instr = self.cpu.next_instr();
            if self.run_mode == RunMode::Debug {
                if self.dump_regs {
                    self.cpu.dump_regs();
                }
                if self.dump_mem {
                    self.cpu.dump_mem(0, 16);
                }
                if self.dump_instr {
                    Color::Green.with(|| print!("~~ Instr: "));
                    match next_instr {
                        None => println!("None"),
                        Some(i) => println!("{}", i),
                    }
                }
            }


            // break loop
            match next_instr {
                None => {
                    self.log_debug("QUT instruction was reached");
                    return Ok((RunOk::QuitInstr, count))
                },
                Some(instr) => {
                    // Execute current instruction
                    self.cpu.cycle(instr);
                    count += 1;
                    match instr {
                        Instr::Invalid(opc) => {
                            self.log_debug("Invalid instruction encountered");
                            return Err(RunErr::InvalidInstr(opc))
                        },
                        _ => {},
                    }
                },
            }
            if count > MAX_CYCLE_COUNT {
                self.log_debug(&*format!(
                    "Max cycle count reached ({})", MAX_CYCLE_COUNT));
                return Err(RunErr::MaxCycle(MAX_CYCLE_COUNT));
            }
        }
    }

    fn parse_commands(&mut self) {
        let mut stdin = io::stdin();
        let mut line = String::new();
        loop {
            line.clear();
            print!("> ");
            io::stdout().flush();
            stdin.read_line(&mut line);
            line.pop(); // remove '\n'
            let args : Vec<_> = line.split(" ").collect();
            match &*args[0] {
                "r" | "run" => {
                    self.paused = false;
                    break;
                },
                "s" | "step" => break,
                "dregs" => self.cpu.dump_regs(),
                "help" => {
                    println!("Available commands:");
                    println!("    help      Print this message");
                    println!("    r, run    Run until breakpoint is reached");
                    println!("    s, step   Execute one step");
                    println!("    dregs     Print register");
                },
                "" => {},
                c @ _ => {
                    println!("Invalid command '{}'. Type 'help'", c);
                },
            }
        }
    }

    pub fn log_debug(&self, m: &str) {
        if self.run_mode == RunMode::Debug {
            println!("~> DEBUG: {}", m);
        }
    }
}
