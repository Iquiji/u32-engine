use image::Pixel;
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;

mod sprites;
use sprites::Sprite;

const W: usize = 80;
const H: usize = 80;

const WINDOW_W: usize = 640;
const WINDOW_H: usize = 640;

const CELL_TO_WINDOW_SCALE: u32 = 8;

fn main() {
    let digits = image::open("digits.png").unwrap().to_rgba();
    let sprites = image::open("sprites.png").unwrap().to_rgba();
    let coin_img = image::open("coin.png").unwrap().to_rgba();

    let mut symbols = HashMap::new();

    symbols.insert('0', Sprite::load_from_image(&digits, 0, 0, 14, 18));
    symbols.insert('1', Sprite::load_from_image(&digits, 18, 0, 14, 18));
    symbols.insert('2', Sprite::load_from_image(&digits, 36, 0, 14, 18));
    symbols.insert('3', Sprite::load_from_image(&digits, 54, 0, 14, 18));
    symbols.insert('4', Sprite::load_from_image(&digits, 72, 0, 14, 18));
    symbols.insert('5', Sprite::load_from_image(&digits, 90, 0, 14, 18));
    symbols.insert('6', Sprite::load_from_image(&digits, 108, 0, 14, 18));
    symbols.insert('7', Sprite::load_from_image(&digits, 126, 0, 14, 18));
    symbols.insert('8', Sprite::load_from_image(&digits, 144, 0, 14, 18));
    symbols.insert('9', Sprite::load_from_image(&digits, 162, 0, 14, 18));

    symbols.insert('o', Sprite::load_from_image(&sprites, 0, 0, 8, 8));
    symbols.insert('t', Sprite::load_from_image(&sprites, 8, 0, 8, 8));
    symbols.insert('c', Sprite::load_from_image(&coin_img, 1, 1, 14, 14));

    let mut map: Vec<bool> = Vec::new();
    map.resize_with(W * H, rand::random);

    //println!("Hello, world!");
    //println!("{:?}",map);
    //print_play_ground(&map);

    let mut window_buffer = Sprite::new(WINDOW_W, WINDOW_H);

    let mut window = Window::new(
        "Conways Game of Life",
        WINDOW_W,
        WINDOW_H,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(1000)));

    let mut paused = false;
    let mut same_frame_counter = 0;
    let mut speed: u32 = 300;
    let mut was_mouse: u32 = 0;

    let mut last_mouse_pos = (0, 0);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window_buffer.clear();
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
        }

        if window.get_mouse_down(minifb::MouseButton::Left) {
            let mouse = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
            let cell_x: usize = (mouse.0 / CELL_TO_WINDOW_SCALE as f32).floor() as usize;
            let cell_y: usize = (mouse.1 / CELL_TO_WINDOW_SCALE as f32).floor() as usize;
            if last_mouse_pos == (cell_x, cell_y) {
                was_mouse = (was_mouse + 1) % 50;
            } else {
                was_mouse = 0;
            }
            last_mouse_pos = (cell_x, cell_y);
            if was_mouse == 0 {
                map[cell_x + W * cell_y] = !map[cell_x + W * cell_y];
            }
        }

        if window.is_key_pressed(Key::Comma, minifb::KeyRepeat::Yes) {
            speed -= 10;
        } else if window.is_key_pressed(Key::Period, minifb::KeyRepeat::Yes) {
            speed += 10;
        }

        if !paused {
            same_frame_counter = (same_frame_counter + 1) % speed;
            if same_frame_counter == 0 {
                map = gen_next_map(map);
            }
        }

        for cell_y in 0..H {
            for cell_x in 0..W {
                let cell = cell_x + cell_y * W;
                if map[cell] {
                    window_buffer.draw_sprite(
                        cell_x * CELL_TO_WINDOW_SCALE as usize,
                        cell_y * CELL_TO_WINDOW_SCALE as usize,
                        symbols.get(&'t').unwrap(),
                    );
                } else {
                    window_buffer.draw_sprite(
                        cell_x * CELL_TO_WINDOW_SCALE as usize,
                        cell_y * CELL_TO_WINDOW_SCALE as usize,
                        symbols.get(&'o').unwrap(),
                    );
                }
            }
        }

        if paused {
            window_buffer.draw_rect(10, 10, 10, 30, 0xAA_AA_AA_AA);
            window_buffer.draw_rect(30, 10, 10, 30, 0xAA_AA_AA_AA);
        }

        // draw current speed
        for (i, digit) in speed.to_string().chars().enumerate() {
            window_buffer.draw_sprite(570 + i * 20, 10, symbols.get(&digit).unwrap());
        }

        window_buffer.draw_sprite(0, 0,symbols.get(&'c').unwrap());
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&window_buffer.buffer, WINDOW_W, WINDOW_H)
            .unwrap();
    }
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
fn draw_from_image(
    buffer: &mut [u32],
    buffer_x: usize,
    buffer_y: usize,
    image: &image::RgbaImage,
    image_x: usize,
    image_y: usize,
    size_x: usize,
    size_y: usize,
) {
    for y in 0..size_y {
        for x in 0..size_x {
            let pixel = image
                .get_pixel((image_x + x) as u32, (image_y + y) as u32)
                .channels();
            //print!("{:?} ", pixel);
            buffer[(x + buffer_x) + (y + buffer_y) * WINDOW_W] = blend(
                u32::from_be_bytes([pixel[3], pixel[0], pixel[1], pixel[2]]),
                buffer[(x + buffer_x) + (y + buffer_y) * WINDOW_W],
            );
        }
        //println!();
    }
    //panic!();
}

fn blend(a: u32, b: u32) -> u32 {
    let a_as_u8 = a.to_be_bytes();
    let b_as_u8 = b.to_be_bytes();
    let alpha_a: f32 = a_as_u8[0] as f32 / 255.0;

    let red = (a_as_u8[1] as f32 * alpha_a + b_as_u8[1] as f32 * (1.0 - alpha_a as f32)) as u8;
    let green = (a_as_u8[2] as f32 * alpha_a + b_as_u8[2] as f32 * (1.0 - alpha_a as f32)) as u8;
    let blue = (a_as_u8[3] as f32 * alpha_a + b_as_u8[3] as f32 * (1.0 - alpha_a as f32)) as u8;
    let alpha: u8 = 0xFF;

    u32::from_be_bytes([alpha, red, green, blue])
}

fn gen_next_map(map: Vec<bool>) -> Vec<bool> {
    let mut next_map: Vec<bool> = Vec::with_capacity(map.len());
    for y in 0..H {
        let y = y as isize;
        for x in 0..W {
            let x = x as isize;
            let iw: isize = W as isize;

            let adjacent_count = map
                .get(((x - 1) + iw * (y - 1)) as usize)
                .map(|x| *x as u32)
                .unwrap_or_default()
                + map
                    .get(((x - 1) + iw * y) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default()
                + map
                    .get(((x - 1) + iw * (y + 1)) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default()
                + map
                    .get((x + iw * (y - 1)) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default()
                + map
                    .get((x + iw * (y + 1)) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default()
                + map
                    .get(((x + 1) + iw * (y - 1)) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default()
                + map
                    .get(((x + 1) + iw * y) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default()
                + map
                    .get(((x + 1) + iw * (y + 1)) as usize)
                    .map(|x| *x as u32)
                    .unwrap_or_default();

            if adjacent_count == 2 && map[x as usize + W * y as usize] || adjacent_count == 3 {
                next_map.push(true);
            } else {
                next_map.push(false);
            }
        }
    }
    next_map
}
