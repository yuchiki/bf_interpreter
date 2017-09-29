#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
enum Instruction {
    Right,
    Left,
    Inc,
    Dec,
    Put,
    Get,     
    JF,
    JB,
}
use Instruction::*;


#[derive(Debug)]
struct PointedVec<T> {
    v: Vec<T>,
    index: usize,
}

impl<T: Copy> PointedVec<T> {
    fn new(v: Vec<T>) -> PointedVec<T> {
        PointedVec::<T> {
            v: v,
            index: 0,
        }
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

    fn update(&mut self, value : T) {
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

const MEMSIZE : usize = 30000;
const PROGRAM : [Instruction; 3] = [Inc, Inc, Put];

fn main() {
    run(&PROGRAM);
}

fn run(code: &[Instruction]) {
    let mem : [u8; MEMSIZE] = [0; MEMSIZE];
    unimplemented!();
}

#[test]
fn pointed_vec_works() {
    let mut pv = PointedVec::new(vec![1,2,3]);
    assert!(pv.read() == 1);
    pv.right();
    assert!(pv.read() == 2);
    pv.left();
    assert!(pv.read() == 1);
    pv.update(100);
    assert!(pv.read() == 100);
}