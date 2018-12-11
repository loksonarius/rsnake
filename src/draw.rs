use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 15.0;

const DIGIT_WIDTH: usize = 4;
const DIGIT_HEIGHT: usize = 4;

const DIGIT_FILL: [[bool; DIGIT_WIDTH * DIGIT_HEIGHT]; 10] = [
    [false,  true,  true, false,
     false,  true, false,  true,
     false,  true, false,  true,
     false, false,  true,  true,
    ], // 0
    [false,  true,  true,  true,
     false, false,  true,  true,
     false, false,  true,  true,
     false, false,  true,  true,
    ], // 1
    [false,  true,  true,  true,
     false, false, false,  true,
     false,  true,  true, false,
     false,  true,  true,  true,
    ], // 2
    [false,  true,  true,  true,
     false, false, false,  true,
     false, false,  true,  true,
     false,  true,  true,  true,
    ], // 3
    [false,  true, false,  true,
     false,  true,  true,  true,
     false,  true,  true,  true,
     false, false, false,  true,
    ], // 4
    [false,  true, true,  true,
     false,  true, false, false,
     false, false,  true,  true,
     false,  true,  true,  true,
    ], // 5
    [false, false,  true,  true,
     false,  true, false, false,
     false,  true,  true,  true,
     false,  true,  true,  true,
    ], // 6
    [false,  true,  true,  true,
     false, false, false,  true,
     false, false,  true,  true,
     false, false, false,  true,
    ], // 7
    [false,  true,  true,  true,
     false,  true, false,  true,
     false,  true,  true,  true,
     false,  true,  true,  true,
    ], // 8
    [false,  true,  true,  true,
     false,  true,  true,  true,
     false, false, false,  true,
     false, false, false,  true,
    ], // 9
];

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
