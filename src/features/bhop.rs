use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;

pub fn run() {
    let toggle_bhop: bool = true;
    println!("Bhop: {}", toggle_bhop);

    while toggle_bhop {

        hp::t_sleep(15); // Sleeping, so we don't eat our CPU

        let m_base;
        unsafe {m_base = mem::BASE};

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration

        if hp::key_state(32) { // If pressed Spacebar (32)
            let flags = mem::read::<u32>(lp + of::m_fFlags);

            if flags == 256 { // If Not in Air
                mem::write::<u32>(m_base + of::dwForceJump, 4);
            }
            else {
                mem::write::<u32>(m_base + of::dwForceJump, 5);
            }
        }
    }
}