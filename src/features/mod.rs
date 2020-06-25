pub mod bhop;
pub mod flash;
pub mod glow;
pub mod radar;
pub mod trigger;

/* Example for a new feature

use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool) {
    if enabled {
        println!("Name: {}", enabled);

        std::thread::spawn(move || {
            launch();
        });
    }
}


fn launch() {
    loop {
        unsafe {
            hp::t_sleep(10);

            let lp = mem::read::<u32>(&mem::BASE + of::dwLocalPlayer);
            if lp == 0 {continue};
        }
    }
}

*/