use crate::sprites::Sprite;
use crate::entity::Entity;
use crate::entity::Interactable;
use minifb::{Key, Window, WindowOptions};

pub struct Player<'a>{
    pub entity: Entity,
    pub vel_x: i32,
    pub vel_y: i32,
    pub may_jump: bool,
    window: &'a Window,
}

impl Interactable for Player<'_>{
    fn update(mut self){
        self.vel_x = 0;
        if let Some(keys) = self.window.get_keys() {
            for t in keys {
                match t {
                    //Key::W => player.y -= 1,
                    Key::A => self.vel_x = -3,
                    //Key::S => player.y += 1,
                    Key::D => self.vel_x = 3,
                    Key::Space => {
                        if self.may_jump{
                            self.vel_y -= 14;
                        }
                    }
                    _ => (),
                }
            }
        };
    }
}