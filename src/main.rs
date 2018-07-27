mod cpu;
mod mem;
use cpu::Cpu;
use mem::Mem;

use std::io;

fn main(){
    println!("{:?}", Cpu::new());
    println!("{:?}", Mem::new());
    println!("{}", Mem::new());
    println!("{}", Mem::test_graphic());
}
