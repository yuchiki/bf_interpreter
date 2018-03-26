#![feature(plugin)]
#![plugin(docopt_macros)]
#![allow(unused_variables)]
#![allow(unused_imports)]


extern crate docopt;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use std::path::Path;
use std::io::BufWriter;
use std::io::Stdout;
use std::io::Write;
use std::fs::File;
use std::io::Read;


mod brainfuck;

docopt!(Args derive Debug, "
bf_interpreter

Usage:
    bf_interpreter (<input> | --expr <expr>) [--output <output>] [--verbose]
    bf_interpreter (--help | --version )

Options:
    -e <expr>, --expr <expr>        Evaluate given expression
    -h, --help                      Show this screen
    -o <output>, --output <output>  Designate output file
    -v, --verbose                   Verbose mode
    -V, --version                   Show version
");


fn main() {
    //let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Args = Args::docopt().deserialize().unwrap_or_else(|e| e.exit());
    let program: &str = &if args.flag_expr.is_empty() {
        let mut file = File::open(Path::new(&args.arg_input)).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        s
    } else {
        args.flag_expr
    };

    let out_chan = if args.flag_output.is_empty() {
        Box::new(std::io::stdout()) as Box<Write>
    } else {
        unimplemented!();
        //Box::new(File::create(args.flag_output).unwrap()) as Box<Write>
    };
    let writer = BufWriter::new(out_chan);

    //println!("{:?}", args);
    brainfuck::BFVM::new(brainfuck::parse(program), args.flag_verbose).run();
}
