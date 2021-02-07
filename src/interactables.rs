use crate::entity::Entity;
use crate::entity::Interactable;
use minifb::{Key, Window};
use rand::Rng;

pub struct Player{
    pub entity: Entity,
}

impl Interactable for Player{
    fn update(&mut self,window: &Window,delta_millis: u32){
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
    fn update(&mut self,_window: &Window,delta_millis: u32){
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