pub fn key_state(key: i32) -> bool {
    unsafe {
        let state: i32 = winapi::um::winuser::GetAsyncKeyState(key).into();
        return (state &0x8000) != 0;
    }
}

pub fn t_sleep(time: u64) {
    use std::{ thread, time };
    thread::sleep(time::Duration::from_millis(time));
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
} 
impl Default for Vector3 {
    fn default() -> Self { Vector3{ x: 0.0, y: 0.0, z: 0.0 } } 
}