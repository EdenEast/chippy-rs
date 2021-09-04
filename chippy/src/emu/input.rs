const KEYPAD_SIZE: usize = 16;

#[derive(Debug, PartialEq)]
pub struct Input {
    pub keys: [bool; KEYPAD_SIZE],
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key {
    Zero = 0x0,
    One = 0x1,
    Two = 0x2,
    Three = 0x3,
    Four = 0x4,
    Five = 0x5,
    Six = 0x6,
    Seven = 0x7,
    Eight = 0x8,
    Nine = 0x9,
    A = 0xA,
    B = 0xB,
    C = 0xC,
    D = 0xD,
    E = 0xE,
    F = 0xF,
}

pub const KEY_LIST: [Key; KEYPAD_SIZE] = [
    Key::Zero,
    Key::One,
    Key::Two,
    Key::Three,
    Key::Four,
    Key::Five,
    Key::Six,
    Key::Seven,
    Key::Eight,
    Key::Nine,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
];

impl Key {
    pub fn as_str(&self) -> &str {
        match *self {
            Key::Zero => "0",
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::A => "A",
            Key::B => "B",
            Key::C => "C",
            Key::D => "D",
            Key::E => "E",
            Key::F => "F",
        }
    }
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: [false; KEYPAD_SIZE],
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn clear(&mut self) {
        self.keys = [false; KEYPAD_SIZE];
    }

    pub fn key_up(&mut self, key: Key) {
        self.keys[key as usize] = false;
    }

    pub fn key_down(&mut self, key: Key) {
        self.keys[key as usize] = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_clear() {
        let input = Input::new();
        assert!(input.keys.iter().all(|k| *k == false))
    }

    #[test]
    fn key_is_pressed() {
        let mut input = Input::new();
        input.keys[0xA] = true;
        assert!(input.keys[0xA]);
    }

    #[test]
    fn set_key_down() {
        let mut input = Input::new();
        let key = Key::A;
        input.key_down(key);
        assert!(input.is_pressed(key as u8));
    }

    #[test]
    fn set_key_up() {
        let mut input = Input::new();
        let key = Key::A;

        input.key_down(key);
        assert!(input.is_pressed(key as u8));

        input.key_up(key);
        assert!(!input.is_pressed(key as u8));
    }
}
