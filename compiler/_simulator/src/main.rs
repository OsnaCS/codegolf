extern crate docopt;

mod cpu;
mod debug;

use debug::{Debugger, RunMode};
use std::fs::File;
use docopt::Docopt;


static USAGE: &'static str = "
Usage:
    simulator [-g] <binfile>
    simulator (-h | --help)

Options:
    -h, --help          Show this message.
    -g, --debug         Execute in debug mode.
";


fn main() {
    let mut debug = Debugger::new();

    let args = match Docopt::new(USAGE).unwrap().parse() {
        Err(e) => {
            println!("Invalid command line arguments:");
            match e {
                // TODO: More error specific information
                _ => {},
            }
            println!("{}", USAGE);
            return;
        }
        Ok(args) => args,
    };

    // get arguments
    let bin_file = args.get_str("<binfile>");
    let run_mode = if args.get_bool("-g") {
        RunMode::Debug
    } else {
        RunMode::Release
    };

    // Need to set runmode first in order to use ´log_debug` correctly
    debug.set_run_mode(run_mode);
    debug.log_debug(&*format!("Run mode: '{:?}'", run_mode));

    debug.log_debug(&*format!("Binary file: '{}'", bin_file));
    let f = File::open(bin_file).unwrap();

    let res = debug.run(f);

    println!("Exited with result {:?} ", res);
}
