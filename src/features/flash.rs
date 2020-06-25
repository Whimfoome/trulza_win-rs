use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool) {
    if enabled {
        println!("NoFlash: {}", enabled);

        std::thread::spawn(move || {
            launch();
        });
    }
}


fn launch() {
    loop {
        unsafe {
            hp::t_sleep(50); // Sleeping, so we don't eat our CPU

            let lp = mem::read::<u32>(&mem::BASE + of::dwLocalPlayer);
            let flashdur = mem::read::<u32>(lp + of::m_flFlashDuration);
    
            if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration
            if flashdur > 0 { // If we are flashed for more than 0 ms, we change it back to 0
                mem::write::<i32>(lp + of::m_flFlashDuration, 0);
            }
        }
    }
}