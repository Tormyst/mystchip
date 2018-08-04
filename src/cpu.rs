use std::fmt;
use mem::Mem;

pub enum ScreenUpdate {
    Yes,
    No
}

pub struct Cpu {
    reg:[u8; 16],
    i: u16,
    pc: u16,
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
        Cpu { i: 0, pc: 0x200, reg: [0; 16] }
    }

    pub fn new_reg(reg: [u8; 16]) -> Cpu {
        Cpu { i: 0, pc: 0x200, reg }
    }

    fn increment_pc(&mut self) {
        self.pc = self.pc.overflowing_add(2).0;
    }


    pub fn cpu_cycle(&mut self, mem: &mut Mem) -> Result<ScreenUpdate, String> {
        //fetch
        let inst_byte_upper = mem.read(self.pc);
        let inst_byte_lower = mem.read(self.pc + 1);

        println!("Instruction fetched: {:02X} {:02X}", inst_byte_upper, inst_byte_lower);
        //decode
        match  high_nibble(inst_byte_upper) {
            0x6 => {  // 6XNN Sets VX to NN 
                self.reg[low_nibble(inst_byte_upper) as usize] = inst_byte_lower; 
                self.increment_pc(); 
                Ok(ScreenUpdate::No) 
            } 
            0xA => {  // ANNN Sets I to NNN
                self.i = fuze(low_nibble(inst_byte_upper), inst_byte_lower); 
                self.increment_pc(); 
                Ok(ScreenUpdate::No) 
            } 
            0xD => {  // DXYN Draws 
                let x = self.reg[low_nibble(inst_byte_upper) as usize];
                let y = self.reg[high_nibble(inst_byte_lower) as usize];
                let n = low_nibble(inst_byte_upper);
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
                self.increment_pc(); 
                Ok(ScreenUpdate::Yes)
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
