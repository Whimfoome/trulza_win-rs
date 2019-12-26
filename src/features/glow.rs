use crate::memory as mem;
use crate::offsets as of;
use crate::helpers as hp;


pub fn ignite(enabled: bool, m_base: u32) {
    let see_team    : bool      = false;
    let color_enemy : hp::Color = hp::Color{r: 1.0, g: 0.0, b: 0.0, a: 0.7};
    let color_team  : hp::Color = hp::Color{r: 0.0, g: 0.0, b: 1.0, a: 0.7};

    if enabled {
        println!("Glow: {}, teammates: {}", enabled, see_team);

        std::thread::spawn(move || {
            launch(m_base, see_team, color_enemy, color_team);
        });
    }
}


pub fn launch(m_base: u32, see_team: bool, color_enemy: hp::Color, color_team: hp::Color) {
    loop {
        hp::t_sleep(20); // Sleeping, so we don't eat our CPU

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        let glowmng = mem::read::<u32>(m_base + of::dwGlowObjectManager);

        if lp == 0 {continue}; // If there is no LocalPlayer (not in-game) skip to next iteration
        for i in 0..66 { // Gets every entity, probably they are less than 65
            let entity = mem::read::<u32>(m_base + of::dwEntityList + (i - 1) * 0x10);
            let myteam = mem::read::<u32>(lp + of::m_iTeamNum);
            let enteam = mem::read::<u32>(entity + of::m_iTeamNum);

            // If Alive and Not Dormant
            if mem::read::<bool>(entity + of::m_iHealth) && !mem::read::<bool>(entity + of::m_bDormant) {
                let curr_glow_index = mem::read::<u32>(entity + of::m_iGlowIndex);

                if myteam != enteam { // Enemy
                    let coloring = glowmng + curr_glow_index * 0x38;
                    mem::write::<f32>(coloring + 0x4, color_enemy.r); // R
                    mem::write::<f32>(coloring + 0x8, color_enemy.g); // G
                    mem::write::<f32>(coloring + 0xC, color_enemy.b); // B
                    mem::write::<f32>(coloring + 0x10, color_enemy.a); // ALPHA
                    mem::write::<bool>(coloring + 0x24, true); // ON-OFF
                    mem::write::<bool>(coloring + 0x25, false); // Square
                }
                else if see_team { // Teammate
                    let coloring = glowmng + curr_glow_index * 0x38;
                    mem::write::<f32>(coloring + 0x4, color_team.r); // R
                    mem::write::<f32>(coloring + 0x8, color_team.g); // G
                    mem::write::<f32>(coloring + 0xC, color_team.b); // B
                    mem::write::<f32>(coloring + 0x10, color_team.a); // ALPHA
                    mem::write::<bool>(coloring + 0x24, true); // ON-OFF
                    mem::write::<bool>(coloring + 0x25, false); // Square
                }
            }
        }
    }
}