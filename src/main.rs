use image::Pixel;
use minifb::{Key, Window, WindowOptions};
use std::collections::HashMap;

mod sprites;
use sprites::Sprite;
mod entity;
use entity::Entity;
mod interactables;
use interactables::Player;

const WINDOW_W: u32 = 640;
const WINDOW_H: u32 = 640;

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
    symbols.insert('G', Sprite::load_from_image(&sprites, 16, 0, 16, 16)); // goal
    symbols.insert('W', Sprite::load_from_image(&sprites, 0, 16, 50, 9)); // u win!
    symbols.insert('c', Sprite::load_from_image(&coin_img, 1, 1, 14, 14));
    symbols.insert('p', Sprite::load_from_image(&sprites, 32, 0, 16, 16));  // player on ground
    symbols.insert('P', Sprite::load_from_image(&sprites, 48, 0, 16, 16));  // player in air

    let mut player = Entity::new_from_sprite(symbols.get(&'p').unwrap().clone(), 0, 0);
    let mut player_vec_x: i32;
    let mut player_vec_y: i32 = 0;

    let mut world: Vec<Entity> = [].to_vec();
    let mut may_jump = false;

    for i in 0..40{
        if i == 18 {
            continue;
        }
        world.push(Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * i, 96));
    }
    world.push(Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * 18, 112));

    world.push(Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * 11, 64));
    world.push(Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * 12, 64));
    world.push(Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * 13, 64));

    for i in 1..4{
        world.push(Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * 32, 96 - 16*i));
    }

    let goal = Entity::new_from_sprite(symbols.get(&'G').unwrap().clone(), 624, 80);
    //println!("Hello, world!");
    //println!("{:?}",map);
    //print_play_ground(&map);

    let mut window_buffer = Sprite::new(WINDOW_W, WINDOW_H);

    let mut window = Window::new("u32 engine", WINDOW_W as usize, WINDOW_H as usize, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(20000)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window_buffer.clear();

        // Key Input
        player_vec_x = 0;
        if let Some(keys) = window.get_keys() {
            for t in keys {
                match t {
                    //Key::W => player.y -= 1,
                    Key::A => player_vec_x = -3,
                    //Key::S => player.y += 1,
                    Key::D => player_vec_x = 3,
                    Key::Space => {
                        if may_jump{
                            player_vec_y -= 14;
                        }
                    }
                    _ => (),
                }
            }
        };

        // draw world:
        for entity in &world{
            window_buffer.draw_sprite(entity.x, entity.y, &entity.texture);
        }

        // gravity:
        if !is_player_colliding_with_entity_vec(&player,&world){
            player_vec_y += 2;
        }

        may_jump = false;
        let mut remaining_vec_y = player_vec_y;
        // X movement
        for _ in 0..player_vec_x.abs(){
            if player_vec_x > 0{
                player.x += 1;
                if is_player_colliding_with_entity_vec(&player,&world){
                    player.x -= 1;
                    break;
                }
            }else{
                player.x -= 1;
                if is_player_colliding_with_entity_vec(&player,&world){
                    player.x += 1;
                    break; 
                }
            }

            // remaining Y movement in each X movement O_
            for _ in 0..remaining_vec_y.abs(){
                if player_vec_y > 0{
                    player.y += 1;
                    remaining_vec_y -= 1;
                    if is_player_colliding_with_entity_vec(&player,&world){
                        player.y -= 1;
                        may_jump = true;
                        break;
                    }
                }else{
                    player.y -= 1;
                    remaining_vec_y += 1;
                    if is_player_colliding_with_entity_vec(&player,&world){
                        player.y += 1;
                        break;
                    }
                }
            }
        }
        // remaining Y movement in each X movement O_
        for _ in 0..remaining_vec_y.abs(){
            if player_vec_y > 0{
                player.y += 1;
                remaining_vec_y -= 1;
                if is_player_colliding_with_entity_vec(&player,&world){
                    player_vec_y = -1;
                    player.y -= 1;
                    may_jump = true;
                    break;
                }
            }else{
                player.y -= 1;
                remaining_vec_y += 1;
                if is_player_colliding_with_entity_vec(&player,&world){
                    player_vec_y = 0;
                    player.y += 1;
                    break;
                }
            }
        }
        
        if player.y + player.height as i32 > WINDOW_H as i32{
            // THE FLOOR IS LAVA!!!1
            player.x = 0;
            player.y = 0;
        }
        // place block where clicked
        if window.get_mouse_down(minifb::MouseButton::Left) {
            let mouse = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
            let cell_x = (mouse.0 / 16.0).floor() as i32;
            let cell_y = (mouse.1 / 16.0).floor() as i32;

            let new = Entity::new_from_sprite(symbols.get(&'g').unwrap().clone(), 16 * cell_x, 16 * cell_y);
            let mut existing = None;
            for (i, entity) in world.iter().enumerate() {
                if rect_rect(new.x, new.y, new.width as i32, new.height as i32, entity.x, entity.y, entity.width as i32, entity.height as i32){
                    existing = Some(i);
                    break;
                }
            }
            if let Some(i) = existing {
                // something was under mouse, kill it! :D
                world.remove(i);
            } else {
                world.push(new);
            }
        }

        window_buffer.draw_sprite(goal.x, goal.y, &goal.texture);
        if is_player_colliding_with_entity_vec(&player, &[goal.clone()]){
            window_buffer.draw_sprite(295, 50, symbols.get(&'W').unwrap());
        }

        // draw player:
        if may_jump {
            window_buffer.draw_sprite(player.x, player.y, &player.texture);
        }else{
            window_buffer.draw_sprite(player.x, player.y, &symbols.get(&'P').unwrap());
        }
        //println!("x:{} y:{}",player.x,player.y);
        //player.y += player_vec_y as i32;
        //println!("{}",rect_rect(player.x, player.y, player.width as i32, player.height as i32, 0, 100, 16, 16));

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&window_buffer.buffer, WINDOW_W as usize, WINDOW_H as usize)
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
fn is_player_colliding_with_entity_vec(player: &Entity,entity_vec: &[Entity]) -> bool{
    for entity in entity_vec{
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
