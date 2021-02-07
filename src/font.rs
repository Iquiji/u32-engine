use std::collections::HashMap;

use crate::sprites::Sprite;

pub struct Font<'a>{
    font_map: HashMap<&'a str,Sprite>
}
impl Font<'_>{
    pub fn init() -> Self{
        let mut font_map: HashMap<&'_ str,Sprite> = HashMap::new();
        let bitmap_png = image::open("16X16-FE-trans.png").unwrap().to_rgba8();
        let symbols_array_helper = [" ","1","BLOCK","BLOCK","BLOCK","BLOCK","BLOCK","BLOCK","BLOCK","BLOCK","BLOCK","BLOCK",".","-",".","BLOCK","0","1","2","3","4","5","6","7","8","9",":",":","BLOCK","=","BLOCK","?","BLOCK","A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"," "];
        for (i,symbol) in symbols_array_helper.iter().enumerate() {
            println!("Init &str:'{}' at x:'{}' y:'{}'",symbol,((i%20)*16) as u32,(i/20) as u32 *16);
            font_map.insert(symbol, Sprite::load_from_image(&bitmap_png, ((i%20)*16) as u32, (i/20) as u32 *16, 16, 16));
        }
        Font{
            font_map
        }
    }
    pub fn get_single(&self,single: &str) -> Sprite{
        match self.font_map.get(&*single.to_ascii_uppercase()){
            Some(sprite) => {
                sprite.to_owned()
            }
            None => {
                println!("Could not find '{}' in Font",single);
                Sprite::new(16, 16)
            }
        }
    }
    pub fn get_string(&self,string: &str) -> Sprite{
        let length_string = string.chars().count();
        let mut base_sprite = Sprite::new(16*length_string as u32, 16);
        
        for (i,char) in string.chars().enumerate() {
            let single_string = char.to_ascii_uppercase().to_string();
            //let sprite_part = self.font_map.get(&*singleString);
            let sprite_part = match self.font_map.get(&*single_string){
                Some(sprite) => {
                    sprite.to_owned()
                }
                None => {
                    println!("Could not find '{}' in Font",single_string);
                    Sprite::new(16, 16)
                }
            };
            base_sprite.draw_sprite(i as i32 *16, 0, &sprite_part);
        }
        //dbg!(&base_sprite);
        base_sprite
    }
}