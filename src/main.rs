use std::fs::File;
use std::io;
use std::process::Command;
use std::time::SystemTime;
use std::time::Duration;

mod cpu;
mod mem;
use cpu::Cpu;
use mem::Mem;
// use std::fmt::Debug;

//use std::io;

#[derive(Debug)]
struct Chip8 {
    cpu: Cpu,
    mem: Mem,
    // Screen
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            mem: Mem::test_graphic()
        }
    }

    pub fn load(&mut self, gamefile: &str) -> Result<usize, io::Error> {
        let f = File::open(gamefile)?;
        self.mem.load(f)
    }
    pub fn cycle(&mut self) {}

    pub fn display(&self) {
        print!("{}", String::from_utf8_lossy(&
        Command::new("clear").output()
            .expect("Failed to clear screen").stdout
        ));
        print!("{}", self.mem);
    }
}

fn main(){
    run();
}

fn run(){
    setupGraphics();
    setupInput();

    let mut chip8 = Chip8::new();
    chip8.load("pong.ch8");
    println!("{:?}", chip8);

    //Time zero
    let mut last_render = SystemTime::now();
    loop {
        chip8.cycle();
        //Wait for 60 hz
        let now = SystemTime::now();
        let difference = now.duration_since(last_render).unwrap();
        if difference >= Duration::from_millis(16) {
            chip8.display();
            last_render = SystemTime::now();
        }
    }
}

fn setupGraphics(){}
fn setupInput(){}
