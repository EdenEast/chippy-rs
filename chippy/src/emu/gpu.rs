pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Gpu {
    pub memory: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub pending_draw: bool,
}

fn index(x: usize, y: usize) -> usize {
    (y % SCREEN_HEIGHT) * SCREEN_WIDTH + (x % SCREEN_WIDTH)
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            memory: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            pending_draw: false,
        }
    }

    pub fn clear(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.set(x, y, false);
            }
        }
        self.pending_draw = false;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.memory[index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let index = index(x, y);
        self.pending_draw |= self.memory[index] != value;
        self.memory[index] = value;
    }

    /// Toggle pixel at location x,y. Returns true if pixel was set
    pub fn toggle(&mut self, x: usize, y: usize, value: bool) -> bool {
        let current = self.get(x, y);
        self.set(x, y, current ^ value);
        current
    }

    pub fn draw(&mut self, x: usize, y: usize, bytes: &[u8]) -> u8 {
        let mut collision = false;
        for yy in 0..bytes.len() {
            for xx in 0..8 {
                let bit = (bytes[yy] >> xx) & 0b1 != 0;
                collision |= self.toggle(x + 7 - xx, y + y, bit);
            }
        }

        match collision {
            true => 1,
            false => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_correct_location() {
        assert_eq!(index(0, 0), 0);
        assert_eq!(index(50, 0), 50);
        assert_eq!(index(0, 1), 64);
        assert_eq!(index(10, 10), 650);
        assert_eq!(index(20, 30), 1940);

        // Wrapping around the screen
        assert_eq!(index(96, 0), 32);
        assert_eq!(index(96, 96), 32);
    }

    #[test]
    fn toggle_pixel() {
        let mut gpu = Gpu::new();
        assert!(gpu.memory.iter().all(|p| !p));

        assert_eq!(gpu.toggle(32, 23, true), false);
        assert_eq!(gpu.toggle(32, 23, true), true);
        assert_eq!(gpu.toggle(32, 23, true), false);
        assert_eq!(gpu.toggle(32, 23, true), true);

        assert_eq!(gpu.toggle(32, 23, false), false);
        assert_eq!(gpu.toggle(32, 23, true), false);
        assert_eq!(gpu.toggle(32, 23, false), true);
    }
}
