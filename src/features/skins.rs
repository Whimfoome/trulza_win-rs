#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use crate::memory as mem;
use crate::offsets as of;
use winapi::ctypes::c_short as short;
use crate::helpers as hp;


/*
* offsets between viewmodel indexes located in the sv_precacheinfo list
* these usually change after new knives are introduced to the game
*/
pub static precache_bayonet_ct: i32 = 89;   // = v_knife_bayonet.mdl - v_knife_default_ct.mdl
pub static precache_bayonet_t: i32 = 65;    // = v_knife_bayonet.mdl - v_knife_default_t.mdl

enum KnifeDefinitionIndex               // id
{
    WeaponKnife = 42,
    WeaponKnifeT = 59,
    WeaponKnifeBayonet = 500,           // 0
    WeaponKnifeFlip = 505,              // 1
    WeaponKnifeGut = 506,               // 2
    WeaponKnifeKarambit = 507,          // 3
    WeaponKnifeM9Bayonet = 508,         // 4
    WeaponKnifeTactical = 509,          // 5
    WeaponKnifeFalchion = 512,          // 6
    WeaponKnifeSurvivalBowie = 514,     // 7
    WeaponKnifeButterfly = 515,         // 8
    WeaponKnifePush = 516,              // 9
    WeaponKnifeUrsus = 519,             // 10
    WeaponKnifeGypsyJackknife = 520,    // 11
    WeaponKnifeStiletto = 522,          // 12
    WeaponKnifeWidowmaker = 523         // 13
}

pub fn run() {
    let toggle_skins: bool = false;
    println!("SkinChanger: {} (UNOPTIMIZED)", toggle_skins);


    let knife_id: i32 = 4;
    let item_def: short = KnifeDefinitionIndex::WeaponKnifeM9Bayonet as short;
    let paint_kit: i32 = 413;
    //
    const ITEM_IDHIGH: i32 = -1;
    const ENTITYQUALITY: i32 = 3;
    const FALLBACKWEAR: f32 = 0.0001;

    let knife_id_offset = if knife_id < 10 {0} else {1};    // precache offset id by 1 for new knives

    let mut cached_player: u32 = 0;
    let mut model_index: i32 = 0;


    while toggle_skins {

        hp::t_sleep(5);

        let m_base;
        unsafe {m_base = mem::BASE};

        let lp = mem::read::<u32>(m_base + of::dwLocalPlayer);
        if lp == 0 {                        // LocalPlayer is not connected to any server
            model_index = 0;
            continue;
        }
        else if lp != cached_player {       // LocalPlayer Changed By Server Switch / Demo Record
            model_index = 0;
            cached_player = lp;
        }

        if paint_kit > 0 && model_index > 0 {
            for i in 0..8 {
                // Handle To Weapon Entity In The Current Slot
                let mut curr_weapon: u32 = mem::read::<u32>(lp + of::hMyWeapons + i * 0x4) &0xFFF;
                curr_weapon = mem::read::<u32>(m_base + of::dwEntityList + (curr_weapon - 1) * 0x10);
                if curr_weapon == 0 {continue};

                // ID Of The Weapon In The Current Slot
                let weapon_id: short = mem::read::<short>(curr_weapon + of::iItemDefinitionIndex);
                let texture = mem::read::<i32>(curr_weapon + of::nFallbackPaintKit);
                
                let mut fallbackpaint = paint_kit;
                match weapon_id {
                    1 => fallbackpaint = 711,
                    2 => fallbackpaint = 396,
                    3 => fallbackpaint = 660,
                    4 => fallbackpaint = 38,
                    7 => fallbackpaint = 707,
                    8 => fallbackpaint = 845,
                    9 => fallbackpaint = 838,
                    10 => fallbackpaint = 626,
                    11 => fallbackpaint = 628,
                    13 => fallbackpaint = 494,
                    14 => fallbackpaint = 547,
                    16 => fallbackpaint = 309,
                    17 => fallbackpaint = 433,
                    19 => fallbackpaint = 283,
                    23 => fallbackpaint = 846,
                    24 => fallbackpaint = 704,
                    25 => fallbackpaint = 654,
                    26 => fallbackpaint = 676,
                    27 => fallbackpaint = 703,
                    28 => fallbackpaint = 514,
                    29 => fallbackpaint = 434,
                    30 => fallbackpaint = 614,
                    32 => fallbackpaint = 389,
                    33 => fallbackpaint = 481,
                    34 => fallbackpaint = 448,
                    35 => fallbackpaint = 3,
                    36 => fallbackpaint = 678,
                    38 => fallbackpaint = 312,
                    39 => fallbackpaint = 287,
                    40 => fallbackpaint = 253,
                    60 => fallbackpaint = 644,
                    61 => fallbackpaint = 504,
                    63 => fallbackpaint = 435,
                    64 => fallbackpaint = 522,
                    _ => {
                        if weapon_id != KnifeDefinitionIndex::WeaponKnife as short && weapon_id != KnifeDefinitionIndex::WeaponKnifeT as short && weapon_id != item_def {continue}
                        else {
                            // Knife-Specific Memory Writes
                            mem::write::<short>(curr_weapon + of::iItemDefinitionIndex, item_def);
                            mem::write::<i32>(curr_weapon + of::nModelIndex, model_index);
                            mem::write::<i32>(curr_weapon + of::iViewModelIndex, model_index);
                            mem::write::<i32>(curr_weapon + of::iEntityQuality, ENTITYQUALITY);
                        }
                    }
                }
                if texture != fallbackpaint && fallbackpaint != 1337 {
                    // Memory Writes Necessary For Both Knives And Other Weapons In Slots
                    mem::write::<i32>(curr_weapon + of::iItemIDHigh, ITEM_IDHIGH);
                    mem::write::<i32>(curr_weapon + of::nFallbackPaintKit, fallbackpaint);
                    mem::write::<f32>(curr_weapon + of::flFallbackWear, FALLBACKWEAR);
                }
            }
        }

        // Handle To Weapon Entity We Are Currently Holding In Hands
        let mut active_wep: u32 = mem::read::<u32>(lp + of::hActiveWeapon) &0xFFF;
        active_wep = mem::read::<u32>(m_base + of::dwEntityList + (active_wep - 1) * 0x10);
        if active_wep == 0 {continue}
        // ID Of Weapon We Are Currently Holding In Hands
        let weapon_id: short = mem::read::<short>(active_wep + of::iItemDefinitionIndex);
        // Viewmodel ID Of The Weapon In Our Hands (Default CT Knife Should Usually Be Around 300)
        let weaponviewmodelid = mem::read::<i32>(active_wep + of::iViewModelIndex);

        // Calculate the correct ModelIndex using the index of Default Knives and the Precache List
        if weapon_id == KnifeDefinitionIndex::WeaponKnife as short && weaponviewmodelid > 0 {
            model_index = weaponviewmodelid + precache_bayonet_ct + 3 * knife_id + knife_id_offset;
        }
        else if weapon_id == KnifeDefinitionIndex::WeaponKnifeT as short && weaponviewmodelid > 0 {
            model_index = weaponviewmodelid + precache_bayonet_t + 3 * knife_id + knife_id_offset;
        }
        else if weapon_id != item_def || model_index == 0 {continue}

        // Handle to Viewmodel Entity we will use to change The Knife Viewmodel Index
        let mut knifeviewmodel = mem::read::<u32>(lp + of::hViewModel) &0xFFF;
        knifeviewmodel = mem::read::<u32>(m_base + of::dwEntityList + (knifeviewmodel - 1) * 0x10);
        if knifeviewmodel == 0 {continue}

        // Change our current viewmodel but only if LocalPlayer is holding a knife in hands
        mem::write::<i32>(knifeviewmodel + of::nModelIndex, model_index);
    }
}