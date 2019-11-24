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