use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;

pub fn run() {
    let toggle_trigger: bool = true;
    println!("TriggerBot: {}", toggle_trigger);

    while toggle_trigger {

        hp::t_sleep(15);
        
        let m_base;
        unsafe {m_base = mem::BASE};

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);

        if lp == 0 {continue};

        if hp::key_state(164) {
            let crosshairid = mem::read::<u32>(lp + of::m_iCrosshairId);
            if crosshairid != 0 && crosshairid <= 64 {
                let entity = mem::read::<u32>(m_base + of::dwEntityList + (crosshairid - 1) * 0x10);
                let myteam = mem::read::<i32>(lp + of::m_iTeamNum);
                let enteam = mem::read::<i32>(entity + of::m_iTeamNum);
                if myteam != enteam {
                    mem::write::<i32>(m_base + of::dwForceAttack, 6);
                }
            }
        }
    }
}