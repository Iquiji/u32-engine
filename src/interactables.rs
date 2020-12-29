use crate::sprites::Sprite;
use crate::entity::Entity;
use crate::entity::Interactable;
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use rand::distributions::{Distribution, Uniform};

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

pub struct Enemy{
    pub entity: Entity,
}
impl Interactable for Enemy{
    fn update(&mut self,_window: &Window){
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4){
            0 => self.entity.y -= 16,
            1 => self.entity.x -= 16,
            2 => self.entity.y += 16,
            3 => self.entity.x += 16,
            x => unreachable!("Random number out of range: {}", x),
        }

    }
}