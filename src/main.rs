extern crate rand;
extern crate piston_window;

use std::fs::File;
use std::io;
use std::process::Command;
use std::time::SystemTime;
use std::time::Duration;
use std::thread::sleep;

mod cpu;
mod mem;
mod display;
use cpu::Cpu;
use mem::Mem;
use display::Display;
// use std::fmt::Debug;

//use std::io;

#[derive(Debug)]
struct Chip8 {
    cpu: Cpu,
    mem: Mem,
    time: SystemTime,
    last_display: SystemTime,
    display: Display,
    // Screen
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            mem: Mem::new(),
            time: SystemTime::now(),
            last_display: SystemTime::now(),
            display: Display::new().unwrap(),
        }
    }

    pub fn load(&mut self, gamefile: &str) -> Result<usize, io::Error> {
        let f = File::open(gamefile)?;
        self.mem.load(f)
    }
    pub fn cycle(&mut self) -> Result<(), String> {
        match self.cpu.cpu_cycle(&mut self.mem)? {
            cpu::ScreenUpdate::Yes => self.display(),
            _ => {}
        };
        let now = SystemTime::now();
        let difference = now.duration_since(self.time).unwrap();
        if difference >= Duration::from_millis(16) {
            self.cpu.updateTime();
            self.time = SystemTime::now();
        }
        Ok(())
    }

    fn display(&mut self) {
        self.display.frame(&self.mem);
    }
}

fn main() {
    run();
}

fn run() {
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

fn setupGraphics() {}
fn setupInput() {}
