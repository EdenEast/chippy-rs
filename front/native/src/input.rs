use chippy::emu::input::Key;
use winit::event::VirtualKeyCode;

#[derive(Debug, Clone, Copy)]
pub enum KeyMapping {
    Qwerty,
    Colemak,
}

impl Default for KeyMapping {
    fn default() -> Self {
        Self::Qwerty
    }
}

pub fn to_emu_key(keycode: &VirtualKeyCode, mapping: KeyMapping) -> Option<Key> {
    match mapping {
        KeyMapping::Qwerty => match keycode {
            VirtualKeyCode::Key1 => Some(Key::One),
            VirtualKeyCode::Key2 => Some(Key::Two),
            VirtualKeyCode::Key3 => Some(Key::Three),
            VirtualKeyCode::Key4 => Some(Key::C),
            VirtualKeyCode::Q => Some(Key::Four),
            VirtualKeyCode::W => Some(Key::Five),
            VirtualKeyCode::E => Some(Key::Six),
            VirtualKeyCode::R => Some(Key::D),
            VirtualKeyCode::A => Some(Key::Seven),
            VirtualKeyCode::S => Some(Key::Eight),
            VirtualKeyCode::D => Some(Key::Nine),
            VirtualKeyCode::F => Some(Key::E),
            VirtualKeyCode::Z => Some(Key::A),
            VirtualKeyCode::X => Some(Key::Zero),
            VirtualKeyCode::C => Some(Key::B),
            VirtualKeyCode::V => Some(Key::F),
            _ => None,
        },
        KeyMapping::Colemak => match keycode {
            VirtualKeyCode::Key1 => Some(Key::One),
            VirtualKeyCode::Key2 => Some(Key::Two),
            VirtualKeyCode::Key3 => Some(Key::Three),
            VirtualKeyCode::Key4 => Some(Key::C),
            VirtualKeyCode::Q => Some(Key::Four),
            VirtualKeyCode::W => Some(Key::Five),
            VirtualKeyCode::F => Some(Key::Six),
            VirtualKeyCode::P => Some(Key::D),
            VirtualKeyCode::A => Some(Key::Seven),
            VirtualKeyCode::R => Some(Key::Eight),
            VirtualKeyCode::S => Some(Key::Nine),
            VirtualKeyCode::T => Some(Key::E),
            VirtualKeyCode::Z => Some(Key::A),
            VirtualKeyCode::X => Some(Key::Zero),
            VirtualKeyCode::C => Some(Key::B),
            VirtualKeyCode::V => Some(Key::F),
            _ => None,
        },
    }
}
