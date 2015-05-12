extern crate docopt;

mod cpu;
mod debug;

use debug::{Debugger, RunMode};
use std::fs::File;


static USAGE: &'static str = "
Usage: simu <binfile>
       simu (-h | --help)

Options:
    -h, --help         Show this message.
";


fn main() {
    let f = File::open("dummy.bin").unwrap();

    let mut debug = Debugger::new();
    let res = debug.run(f, RunMode::Release);

    println!("Exited with result {:?} ", res);
}
