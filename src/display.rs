use std::fmt;

use mem::Mem;

use piston_window::*;

pub struct Display {
    window: PistonWindow,
    gfx: G2dTexture,
}

impl Display {
    pub fn new() -> Result<Display, String> {
        let opengl = OpenGL::V3_2;

        let mut window: PistonWindow = WindowSettings::new("piston: cube", [640, 480])
            .resizable(true)
            .build()
            .unwrap();
        let gfx = Texture::from_memory_alpha(
            &mut window.factory,
            &[255; 640 * 480],
            640,
            480,
            &TextureSettings::new(),
        ).unwrap();
        println!("{:?}", gfx);
        Ok(Display { window, gfx })
    }

    pub fn frame(&mut self, mem: &Mem) {
        while let Some(e) = self.window.next() {
            let gfx = &self.gfx;
            self.window.event(&e);
            self.window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                mem.pixel_locations().into_iter().for_each(|x| {rectangle([1.0; 4], [x.1 as f64 * 10.0 ,x.0 as f64 * 10.0,10.0,10.0],c.view, g)});
            });
            match e {
                UpdateEvent => {break; }
                _ => {}
            }
        }
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display")
    }
}
