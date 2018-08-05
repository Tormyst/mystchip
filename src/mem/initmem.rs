pub fn init() -> [u8; 4096] {
    let mut r = [0x00; 4096];
        r[..80].clone_from_slice(&[   // Font start
            // "0"	Binary	Hex
            0xF0, //  ****
            0x90, //  *  *
            0x90, //  *  *
            0x90, //  *  *
            0xF0, //  ****
            // "1"	Binary	Hex
            0x20, //    * 
            0x60, //   ** 
            0x20, //    * 
            0x20, //    * 
            0x70, //   ***
            // "2"	Binary	Hex
            0xF0, //  ****
            0x10, //     *
            0xF0, //  ****
            0x80, //  *   
            0xF0, //  ****
            // "3"	Binary	Hex
            0xF0, //  ****
            0x10, //     *
            0xF0, //  ****
            0x10, //     *
            0xF0, //  ****
            // "4"	Binary	Hex
            0x90, //  *  *
            0x90, //  *  *
            0xF0, //  ****
            0x10, //     *
            0x10, //     *
            // "5"	Binary	Hex
            0xF0, //  ****
            0x80, //  *   
            0xF0, //  ****
            0x10, //     *
            0xF0, //  ****
            // "6"	Binary	Hex
            0xF0, //  ****
            0x80, //  *   
            0xF0, //  ****
            0x90, //  *  *
            0xF0, //  ****
            // "7"	Binary	Hex
            0xF0, //  ****
            0x10, //     *
            0x20, //    * 
            0x40, //   *  
            0x40, //   *  
            // "8"	Binary	Hex
            0xF0, //  ****
            0x90, //  *  *
            0xF0, //  ****
            0x90, //  *  *
            0xF0, //  ****
            // "9"	Binary	Hex
            0xF0, //  ****
            0x90, //  *  *
            0xF0, //  ****
            0x10, //     *
            0xF0, //  ****
            // "A"	Binary	Hex
            0xF0, //  ****
            0x90, //  *  *
            0xF0, //  ****
            0x90, //  *  *
            0x90, //  *  *
            // "B"	Binary	Hex
            0xE0, //  *** 
            0x90, //  *  *
            0xE0, //  *** 
            0x90, //  *  *
            0xE0, //  *** 
            // "C"	Binary	Hex
            0xF0, //  ****
            0x80, //  *   
            0x80, //  *   
            0x80, //  *   
            0xF0, //  ****
            // "D"	Binary	Hex
            0xE0, //  *** 
            0x90, //  *  *
            0x90, //  *  *
            0x90, //  *  *
            0xE0, //  *** 
            // "E"	Binary	Hex
            0xF0, // ****
            0x80, // *   
            0xF0, // ****
            0x80, // *   
            0xF0, // ****
            // "F"	Binary	Hex
            0xF0, // **** 
            0x80, // *   
            0xF0, // ****
            0x80, // *   
            0x80, // *
            ]);
    r
}
