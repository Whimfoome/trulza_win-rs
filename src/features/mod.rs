pub mod bhop;
pub mod flash;
pub mod glow;
pub mod radar;
pub mod skins;
pub mod trigger;

/* Example for a new feature

use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;

pub fn run() {
    let toggle_name: bool = true;
    println!("Name: {}", toggle_name);

    while toggle_name {

        hp::t_sleep(10);

        let m_base;
        unsafe {m_base = mem::BASE};

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        if lp == 0 {continue};
        
    }
}

*/