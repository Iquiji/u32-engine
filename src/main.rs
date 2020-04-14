use image::Pixel;
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;

mod sprites;
use sprites::Sprite;
mod entity;
use entity::Entity;

const WINDOW_W: usize = 640;
const WINDOW_H: usize = 640;

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

    symbols.insert('g', Sprite::load_from_image(&sprites, 0, 0, 16, 16));
    symbols.insert('c', Sprite::load_from_image(&coin_img, 1, 1, 14, 14));

    let mut player = Entity::new_from_sprite(symbols.get(&'c').unwrap().clone(), 0, 0);
    let mut player_vec_y: f64 = 0.0;

    let mut world: Vec<Entity> = [
        Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 0, 100),
        Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16, 100),
        Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 32, 100)
    ].to_vec();
    //println!("Hello, world!");
    //println!("{:?}",map);
    //print_play_ground(&map);

    let mut window_buffer = Sprite::new(WINDOW_W, WINDOW_H);

    let mut window = Window::new("u32 engine", WINDOW_W, WINDOW_H, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(1000)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window_buffer.clear();

        // Key Input
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::W => player.y -= 1,
                    Key::A => player.x -= 1,
                    Key::S => player.y += 1,
                    Key::D => player.x += 1,
                    Key::Space => {
                        if rectRect(player.x, player.y, player.width as i32, player.height as i32, 0, 100, 16, 16){
                            player_vec_y -= 5.0;
                        }
                    }
                    _ => (),
                }
            }
        });

        // draw player:
        window_buffer.draw_sprite(player.x as usize, player.y as usize, &player.texture);
        window_buffer.draw_sprite(0, 100, symbols.get(&'g').unwrap());
        if !rectRect(player.x, player.y, player.width as i32, player.height as i32, 0, 100, 16, 16){
            player_vec_y += 2.0;
        }

        for i in 0..player_vec_y.abs() as u32{
            println!("{}",i);
            if player_vec_y > 0.0{
                player.y += 1;
                if rectRect(player.x, player.y, player.width as i32, player.height as i32, 0, 100, 16, 16){
                    player_vec_y = 0.0;
                    break;
                }
            }else{
                player.y -= 1;
            }
        }

        //player.y += player_vec_y as i32;
        //println!("{}",rectRect(player.x, player.y, player.width as i32, player.height as i32, 0, 100, 16, 16));

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
fn is_player_colliding_with_entity_vec(player: &Entity,entity_vec: &Vec<Entity>) -> bool{
    for entity in entity_vec{
        if rectRect(player.x, player.y, player.width as i32, player.height as i32, entity.x, entity.y, entity.width as i32, entity.height as i32){
            return true;
        }
    }
    return  false;
}

fn rectRect(
    r1x: i32,
    r1y: i32,
    r1w: i32,
    r1h: i32,
    r2x: i32,
    r2y: i32,
    r2w: i32,
    r2h: i32,
) -> bool {
    // are the sides of one rectangle touching the other?
    if r1x + r1w >= r2x &&    // r1 right edge past r2 left
        r1x <= r2x + r2w &&    // r1 left edge past r2 right
        r1y + r1h >= r2y &&    // r1 top edge past r2 bottom
        r1y <= r2y + r2h
    {
        // r1 bottom edge past r2 top
        return true;
    }
    return false;
}
