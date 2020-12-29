use crate::sprites::Sprite;
use crate::entity::Entity;
use crate::entity::Interactable;
use minifb::{Key, Window, WindowOptions};

pub struct Player{
    pub entity: Entity,
}

impl Interactable for Player{
    fn update(&mut self,window: &Window){
        if let Some(keys) = window.get_keys() {
            for t in keys {
                match t {
                    Key::W => self.entity.y -= 16,
                    Key::A => self.entity.x -= 16,
                    Key::S => self.entity.y += 16,
                    Key::D => self.entity.x += 16,
                    _ => (),
                }
            }
        };
    }
}