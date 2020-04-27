use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool, m_base: &'static u32) {
    if enabled {
        println!("NoFlash: {}", enabled);

        std::thread::spawn(move || {
            launch(&m_base);
        });
    }
}


fn launch(m_base: &u32) {
    loop {
        hp::t_sleep(50); // Sleeping, so we don't eat our CPU

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        let flashdur = mem::read::<u32>(lp + of::m_flFlashDuration);

        if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration
        if flashdur > 0 { // If we are flashed for more than 0 ms, we change it back to 0
            mem::write::<i32>(lp + of::m_flFlashDuration, 0);
        }
    }
}