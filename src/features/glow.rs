use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;

pub fn run() {
    let toggle_glow: bool = true;
    let teammates: bool = false;
    println!("Glow: {}, teammates: {}", toggle_glow, teammates);

    while toggle_glow {

        hp::t_sleep(20);

        let m_base;
        unsafe {m_base = mem::BASE};

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        let glowmng = mem::read::<u32>(m_base + of::dwGlowObjectManager);

        if lp == 0 {continue};

        for i in 0..66 {
            let entity = mem::read::<u32>(m_base + of::dwEntityList + (i - 1) * 0x10);
            let myteam = mem::read::<u32>(lp + of::m_iTeamNum);
            let enteam = mem::read::<u32>(entity + of::m_iTeamNum);

            if mem::read::<bool>(entity + of::m_iHealth) && !mem::read::<bool>(entity + of::m_bDormant) {
                let curr_glow_index = mem::read::<u32>(entity + of::m_iGlowIndex);

                if myteam != enteam {
                    let coloring = glowmng + curr_glow_index * 0x38;
                    mem::write::<f32>(coloring + 0x4, 1.0); // R
                    mem::write::<f32>(coloring + 0x8, 0.0); // G
                    mem::write::<f32>(coloring + 0xC, 0.0); // B
                    mem::write::<f32>(coloring + 0x10, 0.7); // ALPHA
                    mem::write::<bool>(coloring + 0x24, true); // ON-OFF
                    mem::write::<bool>(coloring + 0x25, false); // Square
                }
                else if teammates {
                    let coloring = glowmng + curr_glow_index * 0x38;
                    mem::write::<f32>(coloring + 0x4, 0.0); // R
                    mem::write::<f32>(coloring + 0x8, 0.0); // G
                    mem::write::<f32>(coloring + 0xC, 1.0); // B
                    mem::write::<f32>(coloring + 0x10, 0.7); // ALPHA
                    mem::write::<bool>(coloring + 0x24, true); // ON-OFF
                    mem::write::<bool>(coloring + 0x25, false); // Square
                }
            }
        }
    }
}