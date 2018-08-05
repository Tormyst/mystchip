use std::fmt;
use std::vec::Vec;
use mem::Mem;

extern crate rand;
use rand::prelude::*;

pub enum ScreenUpdate {
    Yes,
    No
}

pub struct Cpu {
    reg:[u8; 16],
    i: u16,
    pc: u16,
    stack: Vec<u16>,
    dt: u8,
    st: u8,
    rand_inst: ThreadRng,
}

fn high_nibble(num: u8) -> u8 {(num  >> 4) & 0x0F}
fn low_nibble(num: u8) -> u8 {num & 0x0F}
fn fuze(high: u8, low: u8) -> u16 {
    let res: u16 = high.into();
    let res = res << 8;
    res + low as u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { 
            i: 0, 
            pc: 0x200, 
            reg: [0; 16], 
            stack: Vec::new(), 
            dt: 0, 
            st: 0,
            rand_inst: thread_rng(),
        }
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.overflowing_add(2).0;
    }

    pub fn updateTime(&mut self) {
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 { self.st -= 1; /* stop buzz if 0 */ }
    }

    pub fn cpu_cycle(&mut self, mem: &mut Mem) -> Result<ScreenUpdate, String> {
        //fetch
        let inst_byte_upper = mem.read(self.pc);
        let inst_byte_lower = mem.read(self.pc + 1);

        self.increment_pc();

        // println!("Instruction fetched: {:02X} {:02X}", inst_byte_upper, inst_byte_lower);
        // decode
        match  high_nibble(inst_byte_upper) {
            0x0 => {
                match inst_byte_upper {
                    0x00 => {
                        match inst_byte_lower {
                            0xEE => { // 00EE Return from function call
                                self.pc = self.stack.pop().unwrap();
                                Ok(ScreenUpdate::No) 
                            }
                            _ => {Err(format!("Instruction {:02X} {:02X}", inst_byte_upper, inst_byte_lower))}
                        }
                    }
                    _ => {Err(format!("Instruction {:02X} {:02X}", inst_byte_upper, inst_byte_lower))}
                }
            }
            0x1 => {  // 1NNN Jump to NNN
                self.pc = fuze(low_nibble(inst_byte_upper), inst_byte_lower); 
                Ok(ScreenUpdate::No) 
            }
            0x2 => {  // 2NNN Call function at NNN
                self.stack.push(self.pc);
                self.pc = fuze(low_nibble(inst_byte_upper), inst_byte_lower); 
                Ok(ScreenUpdate::No) 
            }
            0x3 => {  // 3XNN Branches over next instruction if VX = NN
                if self.reg[low_nibble(inst_byte_upper) as usize] == inst_byte_lower {
                    self.increment_pc();
                }
                Ok(ScreenUpdate::No) 
            }
            0x4 => {  // 4XNN Branches over next instruction if VX != NN
                if self.reg[low_nibble(inst_byte_upper) as usize] != inst_byte_lower {
                    self.increment_pc();
                }
                Ok(ScreenUpdate::No) 
            }
            0x5 => {  // 3XY0 Branches over next instruction if VX = VY
                if self.reg[low_nibble(inst_byte_upper) as usize] == 
                    self.reg[high_nibble(inst_byte_lower) as usize] {
                    self.increment_pc();
                }
                Ok(ScreenUpdate::No) 
            }
            0x6 => {  // 6XNN Sets VX to NN 
                self.reg[low_nibble(inst_byte_upper) as usize] = inst_byte_lower; 
                Ok(ScreenUpdate::No) 
            } 
            0x7 => {  // 7XNN Adds NN to VX
                let x = low_nibble(inst_byte_upper) as usize;
                self.reg[x] = self.reg[x].overflowing_add(inst_byte_lower).0;
                Ok(ScreenUpdate::No) 
            }
            0xA => {  // ANNN Sets I to NNN
                self.i = fuze(low_nibble(inst_byte_upper), inst_byte_lower); 
                Ok(ScreenUpdate::No) 
            } 
            0xC => {  // CXNN Sets VX to RANDOM & NN
                self.reg[low_nibble(inst_byte_upper) as usize] = inst_byte_lower
                    & self.rand_inst.gen::<u8>(); 
                Ok(ScreenUpdate::No) 
            }
            0xD => {  // DXYN Draws 
                let x = self.reg[low_nibble(inst_byte_upper) as usize];
                let y = self.reg[high_nibble(inst_byte_lower) as usize];
                let n = low_nibble(inst_byte_lower);
                let mut ret = false;
                for iter in 0..n {
                    let sprite = mem.read(self.i + (iter as u16));
                    ret |= mem.gfx_write(x, y + iter, sprite);
                }
                if ret {
                    self.reg[0xF] = 1u8;
                }
                else {
                    self.reg[0xF] = 0u8;
                }
                Ok(ScreenUpdate::Yes)
            }
            0xF => {
                match  inst_byte_lower {
                    0x07 => { // FX07 set VX to DT
                        self.reg[low_nibble(inst_byte_upper) as usize] = self.dt;
                        Ok(ScreenUpdate::No)
                    }
                    0x15 => { // FX15 set DT to VX
                        self.dt = self.reg[low_nibble(inst_byte_upper) as usize];
                        Ok(ScreenUpdate::No)
                    }
                    0x18 => { // FX15 set ST to VX
                        self.st = self.reg[low_nibble(inst_byte_upper) as usize];
                        /* if st > 0 start buzz */
                        Ok(ScreenUpdate::No)
                    }
                    0x29 => { // FX29 I is set to the font location indicated by VX
                        self.i = (self.reg[low_nibble(inst_byte_upper) as usize] * 5)
                            as u16;
                        Ok(ScreenUpdate::No)
                    }
                    0x33 => { // FX33 BCD puts VX decimal values into i, i+1 and i+2
                        let bin = self.reg[low_nibble(inst_byte_upper) as usize];
                        mem.write(self.i, bin / 100);
                        mem.write(self.i + 1u16, (bin % 100) / 10);
                        mem.write(self.i + 2u16, bin % 10);
                        Ok(ScreenUpdate::No)
                    }
                    0x65 => {
                       for index in 0..(1 + low_nibble(inst_byte_upper)) as usize {
                           self.reg[index] = mem.read(self.i + index as u16);
                       }
                       Ok(ScreenUpdate::No)
                    }
                    _ => {Err(format!("Instruction {:02X} {:02X}", inst_byte_upper, inst_byte_lower))}
                }
            }
            _ => {Err(format!("Instruction {:02X} {:02X}", inst_byte_upper, inst_byte_lower))}
        }
        //execute
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU {{ I: 0x{:03X}\tPC: 0x{:03X}\treg: [ \n\
        V0: 0x{:02X}\t\
        V1: 0x{:02X}\t\
        V2: 0x{:02X}\t\
        V3: 0x{:02X}\n\
        V4: 0x{:02X}\t\
        V5: 0x{:02X}\t\
        V6: 0x{:02X}\t\
        V7: 0x{:02X}\n\
        V8: 0x{:02X}\t\
        V9: 0x{:02X}\t\
        VA: 0x{:02X}\t\
        VB: 0x{:02X}\n\
        VC: 0x{:02X}\t\
        VD: 0x{:02X}\t\
        VE: 0x{:02X}\t\
        VF: 0x{:02X}\n\
        ] }}", 
        self.i,
        self.pc,
        self.reg[0], 
        self.reg[1],
        self.reg[2],
        self.reg[3],
        self.reg[4],
        self.reg[5],
        self.reg[6],
        self.reg[7],
        self.reg[8],
        self.reg[9],
        self.reg[10],
        self.reg[11],
        self.reg[12],
        self.reg[13],
        self.reg[14],
        self.reg[15],
        )
    }
}

#[cfg(test)]
mod tests {
    use Cpu;
    #[test]
    fn reg_new() {
        let cpu = Cpu::new();
        for reg in cpu.reg.iter() {
            assert_eq!(reg, &0u8);
        }
    }

    #[test]
    fn reg_values() {
        let cpu = Cpu { reg: [55u8; 16] };
        for reg in cpu.reg.iter() {
            assert_eq!(reg, &55u8);
        }
        println!("{:?}", cpu);
    }
}
