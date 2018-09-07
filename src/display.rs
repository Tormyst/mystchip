use std::error;
use std::fmt;
use std::iter;
use std::sync::mpsc;

use mem::Mem;

use image::*;
use piston_window::*;

const off: Rgba<u8> = Rgba {
    data: [0, 0, 0, 255],
};
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
            .vsync(false)
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
            self.data_update(&mut window.encoder);

            if e.render_args().is_some() {
                self.tx.send(::display_message::Input([false; 16]));
            }
            window.draw_2d(&e, |c, g| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                image(&(self.gfx), c.view.scale(10.0, 10.0), g);
            });
        }
    }

    fn data_update(&mut self, encoder: &mut GfxEncoder) {
        while let Ok(message) = self.rx.try_recv() {
            // println!("Display message {:?}", message);
            match message {
                ::cpu_message::Frame(f) => {
                    let mut f = f.into_iter();
                    for col in 0..32 {
                        for row in 0..64 {
                            self.image.put_pixel(
                                row,
                                col,
                                match f.next() {
                                    Some(true) => on,
                                    Some(false) => off,
                                    _ => off,
                                },
                            );
                        }
                    }
                }
                ::cpu_message::Render => {}
                _ => {
                    panic!("Unhandled cpu_message: {:?}", message);
                }
            }
        }
        self.gfx.update(encoder, &self.image);
    }
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display")
    }
}
