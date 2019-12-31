#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use crate::memory as mem;
use crate::offsets as of;
use winapi::ctypes::c_short as short;


pub fn ignite(enabled: bool, m_base: u32) {
    let knife_index : short = 508;
    let knife_skin  : u32   = 413;

    if enabled {
        println!("SkinChanger: {}", enabled);

        std::thread::spawn(move || {
            launch(m_base, knife_index, knife_skin);
        });
    }
}

fn get_weapon_skin(weapon_index: short) -> u32 {
    // set your desired weapon skin values here
    let mut paint: u32 = 0;
    match weapon_index {
        1 => paint = 711,   // Desert Eagle
        2 => paint = 396,   // Dual Berettas
        3 => paint = 660,   // Five-SeveN
        4 => paint = 38,    // Glock-18
        7 => paint = 707,   // AK-47
        8 => paint = 845,   // AUG
        9 => paint = 838,   // AWP
        10 => paint = 626,  // FAMAS
        11 => paint = 628,  // G3SG1
        13 => paint = 494,  // Galil AR
        14 => paint = 547,  // M249
        16 => paint = 309,  // M4A4
        17 => paint = 433,  // MAC-10
        19 => paint = 283,  // P90
        23 => paint = 846,  // MP5-SD
        24 => paint = 704,  // UMP-45
        25 => paint = 654,  // XM1014
        26 => paint = 676,  // PP-Bizon
        27 => paint = 703,  // MAG-7
        28 => paint = 514,  // Negev
        29 => paint = 434,  // Sawed-Off
        30 => paint = 614,  // Tec-9
        32 => paint = 389,  // P2000
        33 => paint = 481,  // MP7
        34 => paint = 448,  // MP9
        35 => paint = 3,    // Nova
        36 => paint = 678,  // P250
        38 => paint = 312,  // SCAR-20
        39 => paint = 287,  // SG 553
        40 => paint = 253,  // SSG 08
        60 => paint = 644,  // M4A1-S
        61 => paint = 504,  // USP-S
        63 => paint = 435,  // CZ75-Auto
        64 => paint = 522,  // R8 Revolver
        _ => (),
    }
    return paint;
}

fn launch(m_base: u32, knife_index: short, knife_skin: u32) {
    const ITEMIDHIGH: i32 = -1;
    const ENTITYQUALITY: i32 = 3;
    const FALLBACKWEAR: f32 = 0.0001;

    let mut model_index: u32 = 0;
    let mut local_player: u32 = 0;

    loop {
        // model_index is different for each server and map
        // below is a simple way to keep track of local base in order to reset model index
        // while also avoiding doing unnecessary extra reads because of the external RPM overhead
        let temp_player = mem::read::<u32>(m_base + of::dwLocalPlayer);
        if temp_player == 0 { // client not connected to any server (works most of the time)
            model_index = 0;
            continue;
        }
        else if temp_player != local_player { // local base changed (new server join/demo record)
            local_player = temp_player;
            model_index = 0;
        }

        while model_index == 0 {
            model_index = knife_index as u32;
        }

        // loop through m_hMyWeapons slots (8 will be enough)
        for i in 0..8 {
            // get entity of weapon in current slot
            let mut current_weapon: u32 = mem::read::<u32>(local_player + of::hMyWeapons + i * 0x4) &0xFFF;
            current_weapon = mem::read::<u32>(m_base + of::dwEntityList + (current_weapon - 1) * 0x10);
            if current_weapon == 0 { continue; }

            let weapon_index: short = mem::read::<short>(current_weapon + of::iItemDefinitionIndex);
            let mut weapon_skin: u32 = get_weapon_skin(weapon_index);

            //for knives, set item and model related properties
            if weapon_index == 42 || weapon_index == 59 || weapon_index == knife_index {
                mem::write::<short>(current_weapon + of::iItemDefinitionIndex, knife_index);
                mem::write::<u32>(current_weapon + of::nModelIndex, model_index);
                mem::write::<u32>(current_weapon + of::iViewModelIndex, model_index);
                mem::write::<i32>(current_weapon + of::iEntityQuality, ENTITYQUALITY);
                weapon_skin = knife_skin;
            }

            if weapon_skin != 0 { // set skin properties
                mem::write::<i32>(current_weapon + of::iItemIDHigh, ITEMIDHIGH);
                mem::write::<u32>(current_weapon + of::nFallbackPaintKit, weapon_skin);
                mem::write::<f32>(current_weapon + of::flFallbackWear, FALLBACKWEAR);
            }
        }

        // get entity of weapon in our hands
        let mut active_weapon: u32 = mem::read::<u32>(local_player + of::hActiveWeapon) &0xFFF;
        active_weapon = mem::read::<u32>(m_base + of::dwEntityList + (active_weapon - 1) * 0x10);
        if active_weapon == 0 { continue; }

        let weapon_index: short = mem::read::<short>(active_weapon + of::iItemDefinitionIndex);
        if weapon_index != knife_index { continue; } // skip if current weapon is not already set to chosen knife

        // get viewmodel entity
        let mut knife_viewmodel: u32 = mem::read::<u32>(local_player + of::hViewModel) &0xFFF;
        knife_viewmodel = mem::read::<u32>(m_base + of::dwEntityList + (knife_viewmodel - 1) * 0x10);
        if knife_viewmodel == 0 { continue; }

        mem::write::<u32>(knife_viewmodel + of::nModelIndex, model_index);
    }

}