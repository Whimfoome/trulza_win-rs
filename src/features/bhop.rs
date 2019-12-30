use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool, m_base: u32) {
    if enabled {
        println!("Bhop: {}", enabled);

        std::thread::spawn(move || {
            launch(m_base);
        });
    }
}


fn launch(m_base: u32) {
    loop {
        hp::t_sleep(15); // Sleeping, so we don't eat our CPU

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration

        while hp::key_state(32) { // If pressed Spacebar (32)
            let flags = mem::read::<u32>(lp + of::m_fFlags);

            if flags == 257 || flags == 263 { // If Not in Air
                mem::write::<u32>(m_base + of::dwForceJump, 5);
                hp::t_sleep(20);
                mem::write::<u32>(m_base + of::dwForceJump, 4);
            }
        }
    }
}