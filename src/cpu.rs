use std::fmt;

pub struct Cpu {
    reg:[u8; 16],
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { reg: [0; 16] }
    }

    pub fn new_reg(reg: [u8; 16]) -> Cpu {
        Cpu { reg }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CPU {{ reg: [ \n\
        V0: 0x{:02x}\t\
        V1: 0x{:02x}\t\
        V2: 0x{:02x}\t\
        V3: 0x{:02x}\n\
        V4: 0x{:02x}\t\
        V5: 0x{:02x}\t\
        V6: 0x{:02x}\t\
        V7: 0x{:02x}\n\
        V8: 0x{:02x}\t\
        V9: 0x{:02x}\t\
        VA: 0x{:02x}\t\
        VB: 0x{:02x}\n\
        VC: 0x{:02x}\t\
        VD: 0x{:02x}\t\
        VE: 0x{:02x}\t\
        VF: 0x{:02x}\n\
        ] }}", 
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
