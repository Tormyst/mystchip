use std::fmt;
use std::io;
use std::io::Read;
use std::fs::File;

mod initmem;

pub struct Mem {
    mem: [u8; 4096],
    gfx: [bool; 64 * 32],
}

fn gfx_offset(row: usize, col: usize) -> Option<usize> {
    if row < 32 && col < 64 {
        Some((row * 64) + col)
    } else {
        None
    }
}

impl Mem {
    pub fn new() -> Mem {
        Mem {
            mem: initmem::init(),
            gfx: [false; 64 * 32],
        }
    }

    pub fn load(&mut self, mut prog: File) -> Result<usize, io::Error> {
        prog.read(&mut self.mem[0x200..])
    }

    pub fn read(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.mem[address as usize] = data;
    }

    pub fn gfx_write(&mut self, x: u8, y: u8, sprite: u8) -> bool {
        let mut res = false;
        // println!("Sprite: {} {} {:02x}", x, y, sprite);
        for b in format!("{:08b}", sprite).chars().enumerate() {
            if b.1 == '1' {
                if let Some(index) = gfx_offset(y as usize, x as usize + b.0) {
                    res |= self.gfx[index];
                    self.gfx[index] = !self.gfx[index];
                }
            }
        }
        res
    }

    fn gfx_read(&self, row: usize, col: usize) -> Option<bool> {
        Some(self.gfx[gfx_offset(row, col)?])
    }
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Mem");
        let mut index = 0u16;
        writeln!(f, "      0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F");
        for val in self.mem.chunks(16) {
            write!(f, "{:04X}: ", index);
            for num in val {
                write!(f, "{:02X} ", num);
            }
            index += 0x10;
            writeln!(f, "");
        }
        write!(f, "")
    }
}

impl fmt::Display for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..32 {
            for col in 0..64 {
                if self.gfx_read(row, col).unwrap() {
                    write!(f, "X").unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}
