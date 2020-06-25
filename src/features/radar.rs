use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool) {
    if enabled {
        println!("Radar: {}", enabled);

        std::thread::spawn(move || {
            launch();
        });
    }
}


fn launch() {
    loop {
        unsafe {
            hp::t_sleep(50); // Sleeping, so we don't eat our CPU

            for i in 0..33 { // Gets every enemy, probably they are less than 32
                let entity = mem::read::<u32>(&mem::BASE + of::dwEntityList + i * 0x10);
                if entity == 0 {continue} // If no entity, skip iter
                if mem::read::<bool>(entity + of::m_bDormant) {continue} // If Dormant, skip iter
    
                let team = mem::read::<i32>(entity + of::m_iTeamNum);
                if team != 2 && team != 3 {continue} // If team is spectator, skip iter
    
                mem::write::<bool>(entity + of::m_bSpotted, true); // Engine Write Spotted
            }
        }
    }
}