use std::fmt;
use std::io;
use std::io::Read;
use std::fs::File;

pub struct Mem {
    mem: [u8; 4096],
    gfx: [bool; 64 * 32]
}

fn gfx_offset(row: usize, col: usize) -> usize {
    (row*64) + col
}

impl Mem {
    pub fn new() -> Mem {
        Mem { mem: [0; 4096], gfx: [false; 64 * 32] }
    }

    pub fn test_graphic() -> Mem {
        let mut gfx = [false; 2048];
        for row in 0..32 {
            for col in 0..64 {
                if col % 2 == row % 2 {
                    gfx[gfx_offset(row, col)] = true;
                }
            }
        }
        Mem { mem: [0; 4096], gfx }
    }

    pub fn load(&mut self, mut prog: File) -> Result<usize, io::Error> {
        prog.read(&mut self.mem[0x200..])
    }

    fn gfx_read(&self, row: usize, col: usize) -> bool {
        self.gfx[gfx_offset(row, col)]
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
                if self.gfx_read(row, col) {
                    write!(f, "X").unwrap();
                }
                else {
                    write!(f, " ").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}
