use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;

pub fn run() {
    let toggle_flash: bool = true;
    println!("NoFlash: {}", toggle_flash);

    while toggle_flash {

        hp::t_sleep(50); // Sleeping, so we don't eat our CPU

        let m_base;
        unsafe {m_base = mem::BASE};

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        let flashdur = mem::read::<u32>(lp + of::m_flFlashDuration);

        if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration
        if flashdur > 0 { // If we are flashed for more than 0 ms, we change it back to 0
            mem::write::<i32>(lp + of::m_flFlashDuration, 0);
        }
    }
}