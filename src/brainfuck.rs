use std;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Right,
    Left,
    Inc,
    Dec,
    Put,
    Get,
    Begin,
    End,
}
use self::Instruction::*;

#[derive(Debug)]
struct PointedVec<T> {
    v: Vec<T>,
    index: usize,
}

impl<T: Copy> PointedVec<T> {
    fn new(v: Vec<T>) -> PointedVec<T> {
        PointedVec::<T> { v: v, index: 0 }
    }

    fn right(&mut self) {
        self.index = self.index + 1;
    }

    fn left(&mut self) {
        self.index = self.index - 1;
    }

    fn read(&mut self) -> T {
        self.v[self.index]
    }

    fn update(&mut self, value: T) {
        self.v[self.index] = value;
    }
}

impl<T> Index<usize> for PointedVec<T> {
    type Output = T;
    fn index<'a>(&'a self, id: usize) -> &'a Self::Output {
        &self.v[id]
    }
}

impl<T> IndexMut<usize> for PointedVec<T> {
    fn index_mut<'a>(&'a mut self, id: usize) -> &'a mut Self::Output {
        &mut self.v[id]
    }
}

pub struct BFVM {
    program: PointedVec<Instruction>,
    memory: PointedVec<u8>,
}

impl BFVM {
    pub fn new(program: Vec<Instruction>) -> BFVM {
        BFVM {
            program: PointedVec::<Instruction>::new(program),
            memory: PointedVec::<u8>::new(vec![0; MEMSIZE]),
        }
    }

    fn is_finished(&mut self) -> bool {
        self.program.v.len() <= self.program.index
    }

    fn r(&mut self) {
        self.program.right();
    }

    fn l(&mut self) {
        self.program.left();
    }

    fn issue(&mut self) {
        println!(
            "{:?}: {:?}@{:?}",
            self.program.index,
            &self.memory.index,
            &self.memory.v[0..20]
        );
        match self.program.read() {
            Right => {
                self.memory.right();
                self.r();
            }
            Left => {
                self.memory.left();
                self.r();
            }
            Inc => {
                let buf = self.memory.read() + 1;
                self.memory.update(buf);
                self.r();
            }
            Dec => {
                let buf = self.memory.read() - 1;
                self.memory.update(buf);
                //self.memory.value() = buf;
                self.r();
            }
            Put => {
                println!("{:?}", self.memory.read() as char);
                self.r();
            }
            Get => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect(
                    "failed to read.",
                );

                self.memory.update(input.chars().nth(0).expect(
                    "input is not a number",
                ) as u8);
                self.r();
            }
            Begin => {
                self.r();
                if self.memory.read() > 0 {
                    return;
                }
                let mut depth = 0;
                loop {
                    let instruction = self.program.read();
                    self.r();
                    match instruction {
                        Begin => depth += 1,
                        End => depth -= 1,
                        _ => {}
                    }
                    if depth < 0 {
                        return;
                    }
                }
            }
            End => {
                if self.memory.read() == 0 {
                    self.r();
                    return;
                }
                self.l();
                let mut depth = 0;
                loop {
                    let instruction = self.program.read();
                    self.l();
                    match instruction {
                        End => depth += 1,
                        Begin => depth -= 1,
                        _ => {}
                    }
                    if depth < 0 {
                        self.r();
                        self.r();
                        return;
                    }
                }
            }
        };
    }

    pub fn run(&mut self) {
        while !self.is_finished() {
            self.issue()
        }
    }
}

const MEMSIZE: usize = 30000;

fn parse_char(character: char) -> Option<Instruction> {
    match character {
        '>' => Some(Right),
        '<' => Some(Left),
        '+' => Some(Inc),
        '-' => Some(Dec),
        '.' => Some(Put),
        ',' => Some(Get),
        '[' => Some(Begin),
        ']' => Some(End),
        _ => None,             // comments are ignored.
    }
}

pub fn parse(code: &str) -> Vec<Instruction> {
    let mut program = Vec::new();
    for character in code.chars() {
        if let Some(instruction) = parse_char(character) {
            program.push(instruction);
        }
    }
    program
}


// Following functions are for tests:

#[test]
fn pointed_vec_works() {
    let mut pv = PointedVec::new(vec![1, 2, 3]);
    assert_eq!(pv.read(), 1);
    pv.right();
    assert_eq!(pv.read(), 2);
    pv.left();
    assert_eq!(pv.read(), 1);
    pv.update(100);
    assert_eq!(pv.read(), 100);
}

#[test]
fn parse_works() {
    assert_eq!(
        parse("+-><test test.,[]"),
        [Inc, Dec, Right, Left, Put, Get, Begin, End]
    );
}

#[test]
fn vm_works() {
    let sum_of_ten = parse("++++++++++ [[>+>+<<-] >>[<<+>>-] <<-]");
    let mut vm = BFVM::new(sum_of_ten);
    vm.run();
    assert_eq!(vm.memory.v[1], 55);
}