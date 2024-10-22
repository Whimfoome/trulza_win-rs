use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool) {
    let keypress  : i32 = 164;
    let delay     : u64 = 10;

    if enabled {
        println!("TriggerBot: {}, delay: {} ms, buttonID: {}", enabled, delay, keypress);

        std::thread::spawn(move || {
            launch(keypress, delay);
        });
    }
}


fn launch(keypress: i32, delay: u64) {
    loop {
        unsafe {
            hp::t_sleep(15); // Sleeping, so we don't eat our CPU

            let lp = mem::read::<u32>(&mem::BASE + of::dwLocalPlayer);
    
            if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration
    
            if hp::key_state(keypress) { // If pressed Left ALT (164)
                let crosshairid = mem::read::<u32>(lp + of::m_iCrosshairId);
                if crosshairid != 0 && crosshairid <= 64 { // When you aim at sb, you get his ID
                    let entity = mem::read::<u32>(&mem::BASE + of::dwEntityList + (crosshairid - 1) * 0x10);
                    let myteam = mem::read::<i32>(lp + of::m_iTeamNum);
                    let enteam = mem::read::<i32>(entity + of::m_iTeamNum);
                    if myteam != enteam {
                        hp::t_sleep(delay);
                        mem::write::<i32>(&mem::BASE + of::dwForceAttack, 6);
                    }
                }
            }
        }
    }
}