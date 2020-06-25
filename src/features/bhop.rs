use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool) {
    if enabled {
        println!("Bhop: {}", enabled);

        std::thread::spawn(move || {
            launch();
        });
    }
}

fn launch() {
    loop {
        unsafe {
            hp::t_sleep(15); // Sleeping, so we don't eat our CPU
            let lp = mem::read::<u32>(&mem::BASE + of::dwLocalPlayer);
            if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration
            
            // if not moving don't jump (in chat for example)
            let p_vel: hp::Vector3 = mem::read::<hp::Vector3>(lp + of::m_vecVelocity);
            if p_vel.x + p_vel.y + p_vel.z == 0.0 {continue}
    
            while hp::key_state(32) { // If pressed Spacebar (32)
                let flags = mem::read::<u32>(lp + of::m_fFlags);
    
                if flags == 257 || flags == 263 { // If Not in Air
                    mem::write::<u32>(&mem::BASE + of::dwForceJump, 5);
                    hp::t_sleep(20);
                    mem::write::<u32>(&mem::BASE + of::dwForceJump, 4);
                }
            }
        }
    }
}