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
            mem: Mem::new()
        }
    }

    pub fn load(&mut self, gamefile: &str) -> Result<usize, io::Error> {
        let f = File::open(gamefile)?;
        self.mem.load(f)
    }
    pub fn cycle(&mut self) -> Result<(), String> {
        match self.cpu.cpu_cycle(&mut self.mem)? {
            cpu::ScreenUpdate::Yes => {self.display()},
            _ => {} 
        };
        Ok(())
    }

    fn display(&self) {
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
    // println!("{:?}", chip8);

    //Time zero
    let mut last_render = SystemTime::now();
    loop {
        let res = chip8.cycle();
        if let Err(str_err) = res {
            println!("Error on {}", str_err);
            break;
        }
        //Wait for 60 hz
        /*
        let now = SystemTime::now();
        let difference = now.duration_since(last_render).unwrap();
        if difference >= Duration::from_millis(16) {
            chip8.display();
            last_render = SystemTime::now();
        }
        */
    }
    println!("{:?}", chip8);
}

fn setupGraphics(){}
fn setupInput(){}
