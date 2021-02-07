use minifb::Window;

use crate::sprites::Sprite;
#[derive(Debug, Clone)]
pub struct Entity {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub texture: Sprite,
}

impl Entity {
    pub fn new_from_sprite(sprite: Sprite, x: i32, y: i32) -> Entity {
        Entity {
            x,
            y,
            width: sprite.width as u32,
            height: sprite.height as u32,
            texture: sprite,
        }
    }
}

pub trait Interactable {
    fn update(&mut self,window: &Window,delta_millis: u32);
}
