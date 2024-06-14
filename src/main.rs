use imageproc::drawing::{draw_text_mut, text_size};
use piston_window::*;
extern crate image as im;

use im::{ImageBuffer, Rgba};
use ab_glyph::{FontArc, PxScale};
fn main() {
    let opengl = OpenGL::V3_2;

    let (width, height) = (720, 720);
    let cell_size = 80;

    let mut window: PistonWindow = WindowSettings::new("Sudoku", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();


    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
            draw_grid(cell_size, width, &mut canvas);
            for y in 0u32..9 {
                for x in 0u32..9 {
                    draw_in_grid(((x + 1) * (y+1)).to_string().as_str(), x, y, cell_size, &mut canvas)
                }
            }
        }
    }
}

fn draw_grid(cell_size: u32, grid_size: u32, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    
    for y in 0u32..grid_size {
        for x in 0u32..grid_size {
            let is_border = x % cell_size == 0 || y % cell_size == 0;
            let block_x = (x / (cell_size * 3)) % 2;
            let block_y = (y / (cell_size * 3)) % 2;
            let is_green_block = (block_x + block_y) % 2 != 0;

            if is_border {
                canvas.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            } else if is_green_block {
                canvas.put_pixel(x, y, Rgba([0, 255, 0, 255]));
            }
        }
    }
}

fn draw_in_grid(text: &str, cell_x: u32, cell_y: u32, cell_size: u32, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>){
    let font_data = include_bytes!("../assets/FiraSans-Regular.ttf") as &[u8];
    let font = FontArc::try_from_slice(&font_data).unwrap();
    let scale = PxScale::from(cell_size as f32 / 2.0);
    let text_size = text_size(scale, &font, text);

    let cell_top_left_x = cell_x * cell_size;
    let cell_top_left_y = cell_y * cell_size;

    // Calculate the position to draw the text so it is centered within the cell
    let x = cell_top_left_x + (cell_size - text_size.0 as u32) / 2;
    let y = cell_top_left_y + (cell_size - text_size.1 as u32) / 3;

    draw_text_mut(canvas, Rgba([0,0,0,255]), x as i32, y as i32, scale, &font, text);
}
