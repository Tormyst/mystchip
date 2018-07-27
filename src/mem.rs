use std::fmt;

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

    fn gfx_read(&self, row: usize, col: usize) -> bool {
        self.gfx[gfx_offset(row, col)]
    }
}

impl fmt::Debug for Mem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mem")
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
