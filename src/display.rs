use std::fmt;
use std::sync::mpsc;
use std::iter;
use std::error;

use mem::Mem;

use piston_window::*;
use image::*;

const off: Rgba<u8> = Rgba { data: [0, 0, 0, 255] };
const on: Rgba<u8> = Rgba { data: [255; 4] };

pub struct Display {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    gfx: G2dTexture,
    render_flag: bool,
    rx: mpsc::Receiver<::cpu_message>,
    tx: mpsc::Sender<::display_message>,
}

impl Display {
    pub fn cycle(rx: mpsc::Receiver<::cpu_message>, tx: mpsc::Sender<::display_message>) {
        let mut window: PistonWindow = WindowSettings::new("MystChip", [640, 320])
            .resizable(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut me = Display::new(&mut window.factory, rx, tx);
        me.window_update(&mut window);
        // me.data_update(&mut window.encoder);
        me.tx.send(::display_message::Die);
    }
    pub fn new(
        factory: &mut GfxFactory,
        rx: mpsc::Receiver<::cpu_message>,
        tx: mpsc::Sender<::display_message>,
    ) -> Display {
        let opengl = OpenGL::V3_2;

        let image = ImageBuffer::new(640, 320);
        let gfx = Texture::from_image(
            factory,
            &image,
            &TextureSettings::new().filter(Filter::Nearest),
        ).unwrap();
        Display {
            image,
            gfx,
            render_flag: false,
            rx,
            tx,
        }
    }

    fn window_update(&mut self, window: &mut PistonWindow) {
        while let Some(e) = window.next() {
            window.event(&e);
            if e.render_args().is_some() {
                self.data_update(&mut window.encoder);
                self.tx.send(::display_message::Input([false; 16]));
            }
            window.draw_2d(&e, |c, g| if self.render_flag {
                clear([0.0, 0.0, 0.0, 1.0], g);
                image(&(self.gfx), c.view.scale(10.0, 10.0), g);
                self.render_flag = false;
            });
        }
    }

    fn data_update(&mut self, encoder: &mut GfxEncoder) {
        while !self.render_flag {
            if let Ok(message) = self.rx.try_recv() {
                // println!("Display message {:?}", message);
                match message {
                    ::cpu_message::Set(y, x, value) => {
                        self.image.put_pixel(
                            x as u32,
                            y as u32,
                            match value {
                                false => off,
                                true => on,
                            },
                        );
                    }
                    ::cpu_message::Render => {
                        self.gfx.update(encoder, &self.image);
                        self.render_flag = true;
                    }
                    _ => {
                        panic!("Unhandled cpu_message: {:?}", message);
                    }
                }
            } else {
                break;
            }
        }
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display")
    }
}
