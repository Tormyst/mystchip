extern crate rand;
extern crate piston_window;
extern crate image;

use std::fs::File;
use std::io;
use std::process::Command;
use std::time::SystemTime;
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use std::sync::mpsc;

mod cpu;
mod mem;
mod display;
use cpu::Cpu;
use mem::Mem;
use display::Display;
use std::fmt::Debug;

const framesize:usize = 64*32;

//use std::io;
pub enum cpu_message {
    Frame([bool; framesize]),
    Render,
}

impl Debug for cpu_message {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            cpu_message::Frame(_) => write!(f, "cpu_message::Frame()"),
            _ => write!(f, "cpu_message::unknown"),
        }
    }
}

#[derive(Debug)]
pub enum display_message {
    Input([bool; 16]),
    Die,
}

#[derive(Debug)]
struct Chip8 {
    cpu: Cpu,
    mem: Mem,
    time: SystemTime,
    cpu_lock: usize,
    to_display: mpsc::Sender<cpu_message>,
    from_display: mpsc::Receiver<display_message>,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let (to_display, from_cpu) = mpsc::channel();
        let (to_cpu, from_display) = mpsc::channel();
        thread::spawn(move || { Display::cycle(from_cpu, to_cpu); });

        Chip8 {
            cpu: Cpu::new(),
            mem: Mem::new(),
            time: SystemTime::now(),
            cpu_lock: 0,
            to_display,
            from_display,
        }
    }

    pub fn load(&mut self, gamefile: &str) -> Result<usize, io::Error> {
        let f = File::open(gamefile)?;
        self.mem.load(f)
    }
    pub fn cycle(&mut self) -> Result<(), String> {
        if self.cpu_lock < 4 {
            match self.cpu.cpu_cycle(&mut self.mem)? {
                cpu::ScreenUpdate::Yes => {self.display(); self.cpu_lock += 1;}
                _ => {}
            };
        }
        let now = SystemTime::now();
        let difference = now.duration_since(self.time).unwrap();
        if difference >= Duration::from_millis(16) {
            self.cpu.updateTime();
            self.time = SystemTime::now();
        }

        for message in self.from_display.try_iter() {
            match message {
                display_message::Die => {
                    ::std::process::exit(0);
                }
                display_message::Input(_) => {
                    self.cpu_lock = 0;
                }
                _ => panic!("Unhandled message: {:?}", message),
            }
        }
        Ok(())
    }

    fn display(&mut self) {
        self.mem.send_frame(&|f| {
            self.to_display.send(cpu_message::Frame(f)).unwrap()
        });
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
