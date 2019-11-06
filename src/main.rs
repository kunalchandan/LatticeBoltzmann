extern crate piston_window;
extern crate image as im;

use piston_window::*;

const S: usize = 512;

//pub struct App {
//    gl: GlGraphics, // OpenGL drawing backend.
//    pub gaussian: im::ImageBuffer<Rgba<u8>, Vec<u8>>
//}
//
//impl App {
//    fn render(&mut self, args: &RenderArgs) {
//        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
//        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
////        let square = rectangle::square(0.0, 0.0, 50.0);
////        let rotation = self.rotation;
////        let (x, y) = (args.window_size[0] / 2.0,
////                                 args.window_size[1] / 2.0);
//
//        self.gl.draw(args.viewport(), |c, gl| {
//            // Clear the screen.
//
//            let mut texture: G2dTexture = Texture::from_image(
//                &mut texture_context,
//                &self.gaussian,
//                &TextureSettings::new()
//            ).unwrap();
//        });
//    }
//
//    fn update(&mut self, args: &UpdateArgs) {
//        // Rotate 2 radians per second.
//        self.rotation += 2.0 * args.dt;
//    }
//}


unsafe fn gaussian_beam(x: u32, y: u32, s_: usize) -> u8 {
    let s: f32 = s_ as f32;
    let x0: f32 = ((x as f32 / s) * 8.0) - 4.0;
    let y0: f32 = ((y as f32 / s) * 8.0) - 4.0;
    let a: u8 = (255.0 * (f32::exp(-1.0 * ((f32::powf(x0, 2.0) / 2.0) + (f32::powf(y0, 2.0) / 2.0))) - 0.0)) as u8;
    return a;
}

fn get_g_beam() -> im::ImageBuffer<im::Rgba<u8>, Vec<u8>>{
    let mut canvas = im::ImageBuffer::new(S as u32, S as u32);
//    let mut grid: [[f32; S]; S] = [[0.0; S]; S];
    for (x, y, pixel) in canvas.enumerate_pixels_mut() {
        *pixel = im::Rgba([unsafe { gaussian_beam(x, y, S) }, 0, 0, 255]);
    }
    return canvas;
}

fn main() {
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
    let mut canvas = get_g_beam();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into()
    };
    let mut texture: G2dTexture = Texture::from_image(
        &mut texture_context,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    while let Some(e) = window.next() {
        if let Some(_) = e.render_args() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }

    // Create a new game and run it.
//    let mut app = App {
//        gl: GlGraphics::new(opengl),
//        gaussian: get_g_beam()
//    };
//
//    let mut events = Events::new(EventSettings::new());
//    while let Some(e) = events.next(&mut window) {
//        if let Some(r) = e.render_args() {
//            app.render(&r);
//        }
//
//        if let Some(u) = e.update_args() {
//            app.update(&u);
//        }
//    }
}
