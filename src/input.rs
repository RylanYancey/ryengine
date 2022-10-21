
use glium::glutin::event::VirtualKeyCode;

pub type KeyCode = VirtualKeyCode;

pub static mut INPUT_CODE: Option<KeyCode> = None;
pub static mut LAST_CODE: Option<KeyCode> = None;

pub struct Input;

impl Input {
    pub fn key_down (code: Option<KeyCode>) -> bool {
        unsafe { INPUT_CODE == code }
    }

    pub fn key_pressed (code: Option<KeyCode>) -> bool {
        unsafe { INPUT_CODE == code && LAST_CODE != code }
    }

    pub fn key_up (code: Option<KeyCode>) -> bool {
        unsafe { LAST_CODE == code && INPUT_CODE != code }
    }
}