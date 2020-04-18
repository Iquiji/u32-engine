use image::Pixel;

#[derive(Debug, Clone)]
pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u32>,
}
impl Sprite {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for xy in 0..self.width * self.height {
            self.buffer[xy as usize] = 0xFF_FF_FF_FF;
        }
    }
    #[allow(dead_code)]
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let deltax = (x1 as f64 - x0 as f64).abs();
        let sx: f64 = if x0 < x1 { 1.0 } else { -1.0 };

        let deltay = -((y1 as f64 - y0 as f64).abs());
        let sy: f64 = if y0 < y1 { 1.0 } else { -1.0 };

        let mut error = deltax + deltay;

        let mut x = x0 as f64;
        let mut y = y0 as f64;

        loop {
            self.buffer[(x as u32 + y as u32* self.width) as usize] = color;
            if x as usize == x1 && y as usize == y1 {
                break;
            }
            let e2 = 2.0 * error;
            if e2 >= deltay {
                error += deltay;
                x += sx;
            }
            if e2 <= deltax {
                error += deltax;
                y += sy;
            }
        }
    }
    #[allow(dead_code)]
    pub fn load_from_image(
        image: &image::RgbaImage,
        start_x: u32,
        start_y: u32,
        size_x: u32,
        size_y: u32,
    ) -> Self {
        let mut symbol: Sprite = Sprite {
            width: size_x,
            height: size_y,
            buffer: Vec::new(),
        };
        for y in start_y..(start_y + size_y) {
            for x in start_x..(start_x + size_x) {
                let pixel = image.get_pixel(x as u32, y as u32).channels();
                symbol
                    .buffer
                    .push(u32::from_be_bytes([pixel[3], pixel[0], pixel[1], pixel[2]]));
            }
        }
        symbol
    }
    #[allow(dead_code)]
    pub fn draw_sprite(&mut self, pos_x: i32, pos_y: i32, sprite: &Sprite) {
        for y in 0..sprite.height as i32{
            if pos_y + y >= self.height as i32 || pos_y + y < 0{
                continue;
            }
            for x in 0..sprite.width as i32{
                if pos_x + x >= self.width as i32 || pos_x + x < 0{
                    continue;
                }
                self.buffer[((x + pos_x) + (y + pos_y) * self.width as i32) as usize] = blend(
                    sprite.buffer[(x + y * sprite.width as i32) as usize],
                    self.buffer[((x + pos_x) + (y + pos_y) * self.width as i32) as usize],
                );
            }
        }
    }
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32) -> Sprite {
        let mut shallow: Sprite = Sprite {
            width,
            height,
            buffer: Vec::new(),
        };
        for _ in 0..shallow.width * shallow.height {
            shallow.buffer.push(0xFF_00_00_00);
        }
        shallow
    }
    #[allow(dead_code)]
    pub fn draw_rect(&mut self, x: u32, y: u32, size_x: u32, size_y: u32, color: u32) {
        for y in y..(y + size_y) {
            for x in x..(x + size_x) {
                self.buffer[(x + y * self.width) as usize] = blend(color, self.buffer[(x + y * self.width) as usize]);
            }
        }
    }
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
