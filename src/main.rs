extern crate image as im;
extern crate nalgebra as na;
extern crate piston_window;

use piston_window::*;

mod lattice;

pub const SCALE_X: u32 = lattice::SCALE_X;
pub const SCALE_Y: u32 = lattice::SCALE_Y;


pub struct App {
    step: u64,
    cursor: [f64; 2],
    lattice: lattice::Lattice
}

impl App {
    fn update(&mut self, args: &UpdateArgs) {
        for i in 0..10 {
            self.step += 1;
            self.lattice.update(self.cursor);
        }
    }
}


unsafe fn gaussian_beam(x: u32, y: u32, s_: u32) -> u8 {
    let s: f32 = s_ as f32;
    let x0: f32 = ((x as f32 / s) * 8.0) - 4.0;
    let y0: f32 = ((y as f32 / s) * 8.0) - 4.0;
    let a: u8 = (255.0 * (f32::exp(-1.0 * ((f32::powf(x0, 2.0) / 2.0) + (f32::powf(y0, 2.0) / 2.0))) - 0.0)) as u8;
    return a;
}


unsafe fn get_gaussian_beam(boltzmann: &lattice::Lattice, canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    println!("GAUSSIAN_BEAM");
    for (x, y, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = im::Rgba([gaussian_beam(x, y, lattice::X), 0, 0, 255]);
    }
}


fn get_density_img(boltzmann: &lattice::Lattice, canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    for x in 0..canvas.dimensions().0 {
        for y in 0..canvas.dimensions().1 {
            *canvas.get_pixel_mut(x, y) = im::Rgba([boltzmann.nodes[(x/SCALE_X) as usize][(y/SCALE_Y) as usize].macro_den as u8, 0, 0, 255]);
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow =
        WindowSettings::new(
        "Lattice-Boltzmann",
        [lattice::X * SCALE_X, lattice::Y * SCALE_Y])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut sim = App {
        step: 0,
        cursor: [0.0; 2],
        lattice: lattice::build_lattice()
    };
    let mut canvas: im::ImageBuffer<im::Rgba<u8>, Vec<u8>>  = im::ImageBuffer::new(lattice::X * SCALE_X, lattice::Y * SCALE_Y);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        e.mouse_cursor(|pos| {
            sim.cursor = pos;
        });

        if let Some(r) = e.render_args() {

            get_density_img(&sim.lattice, &mut canvas);

            let mut texture_context = TextureContext {
                factory: window.factory.clone(),
                encoder: window.factory.create_command_buffer().into()
            };

            let mut texture: G2dTexture = Texture::from_image(
                &mut texture_context,
                &canvas,
                &TextureSettings::new()
            ).unwrap();

            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });

            texture.update(&mut texture_context, &canvas).unwrap();
        }

        if let Some(u) = e.update_args() {
            sim.update(&u);
        }
    }

}
