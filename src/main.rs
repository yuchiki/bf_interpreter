#![allow(unused_variables)]

mod brainfuck;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    brainfuck::BFVM::new(brainfuck::parse(&args.join(" "))).run();
}
