use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;

pub fn run() {
    let toggle_radar: bool = true;
    println!("Radar: {}", toggle_radar);

    while toggle_radar {

        hp::t_sleep(50);

        let m_base;
        unsafe {m_base = mem::BASE};

        for i in 0..33 {
            let entity = mem::read::<u32>(m_base + of::dwEntityList + i * 0x10);
            if entity == 0 {continue}
            if mem::read::<bool>(entity + of::m_bDormant) {continue}

            let team = mem::read::<i32>(entity + of::m_iTeamNum);
            if team != 2 && team != 3 {continue}

            mem::write::<bool>(entity + of::m_bSpotted, true);
        }
    }
}