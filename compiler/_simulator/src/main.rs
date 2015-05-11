extern crate docopt;

mod cpu;

use cpu::Cpu;
use std::fs::File;


static USAGE: &'static str = "
Usage: simu <binfile>
       simu (-h | --help)

Options:
    -h, --help         Show this message.
";


fn main() {
    let f = File::open("dummy.bin").unwrap();


    let mut cpu = Cpu::new();
    cpu.load_program(f).unwrap();

    let mut count = 0;
    while cpu.cycle() && count < 20 {
        cpu.show_mem();
        count += 1;
    }

    println!("Exited after {} cycles", count);
}
