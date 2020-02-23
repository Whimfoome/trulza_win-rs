pub mod bhop;
pub mod flash;
pub mod glow;
pub mod radar;
pub mod trigger;

/* Example for a new feature

use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool, m_base: u32) {
    if enabled {
        println!("Name: {}", enabled);

        std::thread::spawn(move || {
            launch(m_base);
        });
    }
}


fn launch(m_base: u32) {
    loop {
        hp::t_sleep(10);

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        if lp == 0 {continue};
    }
}

*/