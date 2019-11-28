extern crate image as im;
extern crate imageproc as proc;
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
        for i in 0..1 {
            self.step += 1;
            self.lattice.update(self.cursor);
        }
    }
}


fn gaussian_beam(x: u32, y: u32, size: u32) -> (f32, f32, f32) {
    let b: f32 = 0.0;
    let sx: f32 = 2.0;
    let sy: f32 = 2.0;
    let x0: f32 = ((x as f32 / size) * 8.0) - 4.0;
    let y0: f32 = ((y as f32 / size) * 8.0) - 4.0;
    let a: f32 = 255.0;
    let f: f32 = (a * (f32::exp(-1.0 * ((x0.powf(2.0) / (2.0 * sx.powf(2.0))) + (y0.powf(2.0) / (2.0 * sx.powf(2.0))))) - b));
    let df_dx: f32 = f * (x - x0) / sx.powf(2.0);
    let df_dy: f32 = f * (y - y0) / sy.powf(2.0);

    return (f, df_dx, df_dy);
}


unsafe fn get_gaussian_beam_image(boltzmann: &lattice::Lattice, canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    println!("GAUSSIAN_BEAM");
    for (x, y, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = im::Rgba([gaussian_beam(x, y, lattice::X).0 as u8, 0, 0, 255]);
    }
}


fn get_density_img(boltzmann: &lattice::Lattice, canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    for x in 0..canvas.dimensions().0 {
        for y in 0..canvas.dimensions().1 {
            let saturation: u8 = boltzmann.nodes[(x/SCALE_X) as usize][(y/SCALE_Y) as usize].macro_den as u8;
            *canvas.get_pixel_mut(x, y) = im::Rgba([saturation.min(255) as u8, 0, 0, 255]);
        }
    }
}


fn get_velocity_img(boltzmann: &lattice::Lattice, canvas: &mut im::ImageBuffer<im::Rgba<u8>, Vec<u8>>) {
    for x in 0..lattice::X {
        for y in 0..lattice::Y {
            let line = boltzmann.nodes[x as usize][y as usize].macro_vel;
            let mag = line.magnitude() as f32;
            let x0 = ((x * SCALE_X) as f32) + ((SCALE_X / 2) as f32) + line[0];
            let y0 = ((y * SCALE_Y) as f32) + ((SCALE_Y / 2) as f32) + line[1];
            let x1 = ((x * SCALE_X) as f32) + ((SCALE_X / 2) as f32) - line[0];
            let y1 = ((y * SCALE_Y) as f32) + ((SCALE_Y / 2) as f32) - line[1];
            proc::drawing::draw_line_segment_mut(canvas, (x0, y0), (x1, y1), im::Rgba([255, 255, 0, 255]));
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
            get_velocity_img(&sim.lattice, &mut canvas);
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
