pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub type Pixles = [bool; SCREEN_WIDTH * SCREEN_HEIGHT];

#[derive(Debug)]
pub struct Display {
    pub pixels: Pixles,
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
    }

    fn set_pixel(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[(y * SCREEN_WIDTH) + x] = value;
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.pixels[(y * SCREEN_WIDTH) + x]
    }

    fn xor_pixel(&mut self, x: usize, y: usize, new_value: bool) -> bool {
        let current = self.get_pixel(x, y);
        let new_pixel = current ^ new_value;
        self.set_pixel(x, y, new_pixel);
        current && !new_pixel
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0;

        sprite.iter().enumerate().for_each(|(line_number, line)| {
            if line_number + y < SCREEN_HEIGHT {
                format!("{:08b}", line)
                    .chars()
                    .map(|char| char == '1')
                    .enumerate()
                    .for_each(|(column_number, pixel)| {
                        if column_number + x < SCREEN_WIDTH {
                            if self.xor_pixel(x + column_number, y + line_number, pixel) {
                                collision = 1;
                            }
                        }
                    });
            }
        });
        collision
    }
}
