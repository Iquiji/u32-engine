use image::Pixel;

pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}
impl Sprite {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        for xy in 0..self.width * self.height {
            self.buffer[xy] = 0xFF_00_00_00;
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
            self.buffer[x as usize + y as usize * self.width] = color;
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
        start_x: usize,
        start_y: usize,
        size_x: usize,
        size_y: usize,
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
    pub fn draw_sprite(&mut self, pos_x: usize, pos_y: usize, sprite: &Sprite) {
        for y in 0..sprite.height {
            for x in 0..sprite.width {
                self.buffer[(x + pos_x) + (y + pos_y) * self.width] = blend(
                    sprite.buffer[x + y * sprite.width],
                    self.buffer[(x + pos_x) + (y + pos_y) * self.width],
                );
            }
        }
    }
    pub fn new(width: usize, height: usize) -> Sprite {
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
    pub fn draw_rect(&mut self, x: usize, y: usize, size_x: usize, size_y: usize, color: u32) {
        for y in y..(y + size_y) {
            for x in x..(x + size_x) {
                self.buffer[x + y * self.width] = blend(color, self.buffer[x + y * self.width]);
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
