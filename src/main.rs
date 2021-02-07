use image::Pixel;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::collections::HashMap;

mod sprites;
use sprites::Sprite;
mod entity;
use entity::{Entity, Interactable};
mod interactables;
use interactables::{Player,Enemy};
use std::time::Instant;
mod font;
use font::Font;

const WINDOW_W: u32 = 640;
const WINDOW_H: u32 = 640;

fn main() {
    let base_font = Font::init();

    let digits = image::open("digits.png").unwrap().to_rgba8();
    let sprites = image::open("sprites.png").unwrap().to_rgba8();
    let coin_img = image::open("coin.png").unwrap().to_rgba8();

    let mut symbols = HashMap::new();

    // Old number loading: // new in font.rs
    // symbols.insert('0', Sprite::load_from_image(&digits, 0, 0, 14, 18));
    // symbols.insert('1', Sprite::load_from_image(&digits, 18, 0, 14, 18));
    // symbols.insert('2', Sprite::load_from_image(&digits, 36, 0, 14, 18));
    // symbols.insert('3', Sprite::load_from_image(&digits, 54, 0, 14, 18));
    // symbols.insert('4', Sprite::load_from_image(&digits, 72, 0, 14, 18));
    // symbols.insert('5', Sprite::load_from_image(&digits, 90, 0, 14, 18));
    // symbols.insert('6', Sprite::load_from_image(&digits, 108, 0, 14, 18));
    // symbols.insert('7', Sprite::load_from_image(&digits, 126, 0, 14, 18));
    // symbols.insert('8', Sprite::load_from_image(&digits, 144, 0, 14, 18));
    // symbols.insert('9', Sprite::load_from_image(&digits, 162, 0, 14, 18));

    symbols.insert('g', Sprite::load_from_image(&sprites, 0, 0, 16, 16)); // ground
    symbols.insert('G', Sprite::load_from_image(&sprites, 16, 0, 16, 16)); // goal
    symbols.insert('W', Sprite::load_from_image(&sprites, 0, 16, 50, 9)); // u win!
    symbols.insert('c', Sprite::load_from_image(&coin_img, 1, 1, 14, 14));
    symbols.insert('p', Sprite::load_from_image(&sprites, 32, 0, 16, 16));  // player on ground
    symbols.insert('P', Sprite::load_from_image(&sprites, 48, 0, 16, 16));  // player in air

    let mut window_buffer = Sprite::new(WINDOW_W, WINDOW_H);

    let mut window = Window::new("u32 engine zrpg", WINDOW_W as usize, WINDOW_H as usize, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(20000)));

    let mut player = Player{
        entity: Entity::new_from_sprite(symbols.get(&'p').unwrap().clone(), 0, 0),
    };

    let mut rng = rand::thread_rng();
    let mut enemies: Vec<Enemy> = vec![];
    for _ in 0..5{
        enemies.push(Enemy{
            entity: Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), rng.gen_range(10..20)*16, rng.gen_range(10..20)*16)
        });
    }
    let mut coin: Entity = Entity::new_from_sprite(symbols.get(&'c').unwrap().clone(), 15*16, 15*16);

    let mut score: u64 = 0;
    let mut i: u64 = 0;

    let mut time_frame = Instant::now();
    let mut counter: u32 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        const MAX_UPDATE_EVERY : u64 = 15; // ~?/sec
        window_buffer.clear();

        let delta_millis: u32 = time_frame.elapsed().as_millis() as u32;
        time_frame = Instant::now();
        // println!("frame_time: {}, fps: {}",delta_millis,1000.0 / delta_millis as f32);

        // Key Input
        if i == 0 {
            player.update(&window,delta_millis);
            for enemy in &mut enemies{
                enemy.update(&window,delta_millis);
            }
        }
        i = (i+1) % MAX_UPDATE_EVERY;

        if is_player_colliding_with_entities(&player.entity, enemies.iter().map(|enemy|{&enemy.entity})){
            player.entity.x = 0;
            player.entity.y = 0;
        }
        if is_player_colliding_with_entities(&player.entity, std::iter::once(&coin)){
            score += 1;
            coin.x = rng.gen_range(0..(640/16))*16;
            coin.y = rng.gen_range(0..(640/16))*16;
            dbg!(score);
        }
        // draw world:
        for enemy in &enemies{
            window_buffer.draw_sprite(enemy.entity.x, enemy.entity.y, &enemy.entity.texture);
        }

        window_buffer.draw_sprite(coin.x, coin.y, &symbols.get(&'c').unwrap());
        draw_score(&mut window_buffer,score,&base_font);

        window_buffer.draw_sprite(player.entity.x, player.entity.y, &symbols.get(&'P').unwrap());


        window_buffer.draw_sprite(0, 0, &base_font.get_string(&counter.to_string()));
        window_buffer.draw_sprite(0, 16, &base_font.get_string(&("fps: ".to_owned() + &(1000/delta_millis).to_string())));
        // if u_win{
        //     window_buffer.draw_sprite(20*16, 5*16, &symbols.get(&'W').unwrap())
        // }
        //println!("x:{} y:{}",player.x,player.y);
        //player.y += player_vec_y as i32;
        //println!("{}",rect_rect(player.x, player.y, player.width as i32, player.height as i32, 0, 100, 16, 16));

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&window_buffer.buffer, WINDOW_W as usize, WINDOW_H as usize)
            .unwrap();
        counter += 1;
    }
}

fn draw_score(sprite: &mut Sprite, score: u64, font: &Font) {
    //sprite.draw_sprite(WINDOW_W as i32- 32, 16, symbols.get(&score.to_string().chars().next().unwrap()).unwrap());
    sprite.draw_sprite(WINDOW_W as i32- 9*16, 16, &font.get_string(&("Score: ".to_owned() + &score.to_string())));
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
            buffer[(x + buffer_x) + (y + buffer_y) * WINDOW_W as usize] = blend(
                u32::from_be_bytes([pixel[3], pixel[0], pixel[1], pixel[2]]),
                buffer[(x + buffer_x) + (y + buffer_y) * WINDOW_W as usize],
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
fn is_player_colliding_with_entities<'a>(player: &Entity, entities: impl Iterator<Item=&'a Entity>) -> bool{
    for entity in entities{
        if rect_rect(player.x, player.y, player.width as i32, player.height as i32, entity.x, entity.y, entity.width as i32, entity.height as i32){
            return true;
        }
    }
    false
}

#[allow(clippy::too_many_arguments)]
fn rect_rect(
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
    if r1x + r1w > r2x &&     // r1 right edge past r2 left
        r1x < r2x + r2w &&    // r1 left edge past r2 right
        r1y + r1h > r2y &&    // r1 top edge past r2 bottom
        r1y < r2y + r2h       // r1 bottom edge past r2 top
    {
        return true;
    }
    false
}
