use imageproc::drawing::{draw_text_mut, text_size};
use piston_window::*;
extern crate image as im;

use ab_glyph::{FontArc, PxScale};
use im::{ImageBuffer, Rgba};
fn main() {

    // window, texture and board setup

    let opengl = OpenGL::V3_2;

    let (width, height) = (720, 720);
    let cell_size = 80;

    let mut board = [[3,0,6,5,0,8,4,0,0],
                                    [5,2,0,0,0,0,0,0,0],
                                    [0,8,7,0,0,0,0,3,1],
                                    [0,0,3,0,1,0,0,8,0],
                                    [9,0,0,8,6,3,0,0,5],
                                    [0,5,0,0,9,0,6,0,0],
                                    [1,3,0,0,0,0,2,5,0],
                                    [0,0,0,0,0,0,0,7,4],
                                    [0,0,5,2,0,6,3,0,0]] as [[u8; 9]; 9];

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


    // running the backtracking algorithm with initial parameters
    // TODO: move window related arguments into a struct
    solve(&mut board, 0, 0, &mut window, &mut texture, &mut texture_context, &mut canvas, cell_size, width);

    // keep rendering the window until escape is pressed or program is ended
    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
    }
}

fn draw_grid(cell_size: u32, grid_size: u32, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for y in 0u32..grid_size {
        for x in 0u32..grid_size {
            let is_border = x % cell_size == 0 || y % cell_size == 0;

            let block_x = (x / (cell_size * 3)) % 2;
            let block_y = (y / (cell_size * 3)) % 2;

            // every 3 blocks are green and then white and so on
            let is_green_block = (block_x + block_y) % 2 != 0;

            if is_border {
                canvas.put_pixel(x, y, Rgba([0, 0, 0, 255]));
            } else if is_green_block {
                canvas.put_pixel(x, y, Rgba([0, 255, 0, 255]));
            }
        }
    }
}

fn draw_in_grid(
    text: &str,
    cell_x: u32,
    cell_y: u32,
    cell_size: u32,
    canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
) {

    // font setup
    let font_data = include_bytes!("../assets/FiraSans-Regular.ttf") as &[u8];
    let font = FontArc::try_from_slice(&font_data).unwrap();
    let scale = PxScale::from(cell_size as f32 / 2.0);
    let text_size = text_size(scale, &font, text);

    let cell_top_left_x = cell_x * cell_size;
    let cell_top_left_y = cell_y * cell_size;

    // Calculate the position to draw the text so it is centered within the cell
    // theoretically there should be / 2 in both places but it didn't center it vertically
    let x = cell_top_left_x + (cell_size - text_size.0 as u32) / 3;
    let y = cell_top_left_y + (cell_size - text_size.1 as u32) / 2;

    draw_text_mut(
        canvas,
        Rgba([0, 0, 0, 255]),
        y as i32,
        x as i32,
        scale,
        &font,
        text,
    );
}

fn is_valid(board: &mut [[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {

    // if value is present in the row or column return false
    for i in 0..9 {
        if board[row][i] == num || board[i][col] == num {
            return false;
        };
    }

    // calculate the starting index of the subgrid that the checked number is in
    let start_row = row - row % 3;
    let start_col = col - col % 3;

    // check for number in subgrid
    for i in 0..3 {
        for j in 0..3 {
            if board[i + start_row][j + start_col] == num {
                return false;
            }
        }
    }

    true
}

fn solve(board: &mut [[u8; 9]; 9], mut row: usize, mut col: usize, window: &mut PistonWindow, texture: &mut G2dTexture, texture_context: &mut G2dTextureContext, canvas: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cell_size: u32, width: u32) -> bool {

    // render board every time the function is called (there probably is a more efficient solution)
    if let Some(e) = window.next() {
        if e.render_args().is_some() {
            texture.update(texture_context, &canvas).unwrap();
            
            for x in 0..width {
                for y in 0..width {
                    canvas.put_pixel(x, y, Rgba([255,255,255,255]))
                }
            }

            window.draw_2d(&e, |c, g, device| {
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(texture, c.transform, g);
            });
            draw_grid(cell_size, width, canvas);
            for x in 0..9 {
                for y in 0..9 {
                    draw_in_grid(
                        board[x][y].to_string().as_str(),
                        x as u32,
                        y as u32,
                        cell_size,
                        canvas,
                    )
                }
            }
        }
    }

    // if the algorithm has reached the bottom right of the grid, it means that everything is solved
    if row == 8 && col == 9 {
        return true;
    }

    // move to next row if column is out of bounds
    if col == 9 {
        row += 1;
        col = 0;
    }

    // if value is not set call the function recursively with every valid number
    if board[row][col] == 0 {
        for i in 1..10 {
            if is_valid(board, row, col, i) {
                board[row][col] = i;
                if solve(board, row, col, window, texture, texture_context, canvas, cell_size, width) {
                    return true;
                }
            }
            // code gets to this point if no valid solutions exist, so it resets the spot
            board[row][col] = 0
        }
        return false;
    } else {
        // if the value is set, move to the next value
        return solve(board, row, col + 1, window, texture, texture_context, canvas, cell_size, width)
    }
}
