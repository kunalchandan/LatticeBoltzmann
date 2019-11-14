extern crate image as im;
extern crate nalgebra as na;
extern crate piston_window;

use opengl_graphics::GlGraphics;
use piston_window::*;
use crate::lattice::{Lattice, build_lattice};

mod lattice;

const S: usize = 256;

pub struct App {
    size: u32
}

impl App {
    fn update(&mut self, args: &UpdateArgs) {
        self.size += 1;
    }
}


unsafe fn gaussian_beam(x: u32, y: u32, s_: u32) -> u8 {
    let s: f32 = s_ as f32;
    let x0: f32 = ((x as f32 / s) * 8.0) - 4.0;
    let y0: f32 = ((y as f32 / s) * 8.0) - 4.0;
    let a: u8 = (255.0 * (f32::exp(-1.0 * ((f32::powf(x0, 2.0) / 2.0) + (f32::powf(y0, 2.0) / 2.0))) - 0.0)) as u8;
    return a;
}

fn get_g_beam(i: u32) -> im::ImageBuffer<im::Rgba<u8>, Vec<u8>>{
    let mut canvas = im::ImageBuffer::new(S as u32, S as u32);
//    let mut grid: [[f32; S]; S] = [[0.0; S]; S];
    for (x, y, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = im::Rgba([unsafe { gaussian_beam(x, y, i) }, 0, 0, 255]);
    }
    return canvas;
}

fn main() {
    println!("{}", 5);
    let mut latt: Lattice = build_lattice();
    for x in latt.nodes.iter_mut() {
        for y in x.iter_mut() {
            y.calc_macro_den();
            y.calc_macro_vel();
        }
    }
    // println!("{}", vector::Vect2{x: 1.2, y: 3.0} + vector::Vect2{x: 2.0, y: 3.0});
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow =
        WindowSettings::new(
        "spinning-square",
        [S as u32, S as u32])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut sim = App {
        size: 2
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            let img = get_g_beam(sim.size);

            let mut texture_context = TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into()
            };
            let mut texture: G2dTexture = Texture::from_image(
                &mut texture_context,
                &img,
                &TextureSettings::new()
            ).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
            texture.update(&mut texture_context, &img).unwrap();
        }

        if let Some(u) = e.update_args() {
            sim.update(&u);
        }
    }

}
