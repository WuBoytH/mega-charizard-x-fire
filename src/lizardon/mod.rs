use {
    std::collections::HashSet,
    smash::{
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    once_cell::sync::Lazy,
    walkdir::*
};

pub static MEGAZARD_SLOTS: Lazy<HashSet<i32>> = Lazy::new(|| {
    let mut vec = HashSet::new();
    for x in WalkDir::new("mods:/fighter/plizardon/model/body").min_depth(1).into_iter().flatten() {
        if x.file_type().is_file() && x.path().ends_with("zardx.marker") {
            let str = x.path().to_str().unwrap().replace("/zardx.marker", "").replace("mods:/fighter/plizardon/model/body/c", "");
            let num : i32 = str.parse().unwrap();
            vec.insert(num);
        }
    }
    vec
});

#[skyline::hook(offset = 0x34c99d0)]
pub unsafe extern "C" fn zard_set_flame(vtable: u64, fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let costume_slot = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    Lazy::force(&MEGAZARD_SLOTS);
    if !MEGAZARD_SLOTS.contains(&costume_slot) {
        return original!()(vtable, fighter);
    }
    // if costume_slot != 7 {
    //     return original!()(vtable, fighter);
    // }
    EffectModule::kill_kind(boma, Hash40::new("plizardon_tail_fire"), true, true);
    EffectModule::req_follow(
        boma,
        Hash40::new("plizardon_tail_fire"),
        Hash40::new("s_fire1"),
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        1.0,
        true,
        0x40000,
        0,
        -1,
        0,
        0,
        false,
        false
    );
    // let tail_fire_handle = EffectModule::get_last_handle(boma) as u32;
    // EffectModule::set_rgb(boma, tail_fire_handle, 0.015, 0.05, 5.0);
    EffectModule::req_follow(
        boma,
        Hash40::new("plizardon_tail_fire"),
        Hash40::new("mouthfirel"),
        &Vector3f{x: 0.5, y: 0.5, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: -90.0},
        0.6,
        false,
        0x40000,
        0,
        -1,
        0,
        0,
        false,
        false
    );
    // let mouth_fire_l = EffectModule::get_last_handle(boma) as u32;
    // EffectModule::set_rgb(boma, mouth_fire_l, 0.01, 0.05, 5.0);
    EffectModule::req_follow(
        boma,
        Hash40::new("plizardon_tail_fire"),
        Hash40::new("mouthfirer"),
        &Vector3f{x: 0.5, y: 0.5, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: -90.0},
        0.6,
        false,
        0x40000,
        0,
        -1,
        0,
        0,
        false,
        false
    );
    // let mouth_fire_r = EffectModule::get_last_handle(boma) as u32;
    // EffectModule::set_rgb(boma, mouth_fire_r, 0.01, 0.05, 5.0);
}

#[skyline::hook(offset = 0xf93c10)]
pub unsafe extern "C" fn zard_set_hide_flame(fighter: &mut Fighter) {
    let boma = fighter.battle_object.module_accessor;
    let costume_slot = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    Lazy::force(&MEGAZARD_SLOTS);
    if !MEGAZARD_SLOTS.contains(&costume_slot) {
        return original!()(fighter);
    }
    // if costume_slot != 7 {
    //     return original!()(fighter);
    // }
    WorkModule::set_int(boma, 0, *FIGHTER_PLIZARDON_INSTANCE_WORK_ID_INT_TAIL_FIRE_EFFECT_HANDLE);
    EffectModule::kill_kind(boma, Hash40::new("plizardon_tail_fire"), true, true);
    EffectModule::req_follow(
        boma,
        Hash40::new("plizardon_tail_fire"),
        Hash40::new("s_fire1"),
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        1.0,
        false,
        0x40000,
        0,
        -1,
        0,
        0,
        false,
        false
    );
    let tail_fire_handle = EffectModule::get_last_handle(boma) as u32;
    WorkModule::set_int(boma, tail_fire_handle as i32, *FIGHTER_PLIZARDON_INSTANCE_WORK_ID_INT_TAIL_FIRE_EFFECT_HANDLE);
    // EffectModule::set_rgb(boma, tail_fire_handle, 0.01, 0.05, 5.0);
    EffectModule::req_follow(
        boma,
        Hash40::new("plizardon_tail_fire"),
        Hash40::new("mouthfirel"),
        &Vector3f{x: 0.5, y: 0.5, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: -90.0},
        0.6,
        false,
        0x40000,
        0,
        -1,
        0,
        0,
        false,
        false
    );
    let mouth_fire_l = EffectModule::get_last_handle(boma) as u32;
    // EffectModule::set_rgb(boma, mouth_fire_l, 0.01, 0.05, 5.0);
    EffectModule::req_follow(
        boma,
        Hash40::new("plizardon_tail_fire"),
        Hash40::new("mouthfirer"),
        &Vector3f{x: 0.5, y: 0.5, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: -90.0},
        0.6,
        false,
        0x40000,
        0,
        -1,
        0,
        0,
        false,
        false
    );
    let mouth_fire_r = EffectModule::get_last_handle(boma) as u32;
    // EffectModule::set_rgb(boma, mouth_fire_r, 0.01, 0.05, 5.0);
    if tail_fire_handle != 0
    && WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SPYCLOAK) {
        if tail_fire_handle != 0 {
            EffectModule::set_visible(boma, tail_fire_handle, false);
        }
        if mouth_fire_l != 0 {
            EffectModule::set_visible(boma, mouth_fire_l, false);
        }
        if mouth_fire_r != 0 {
            EffectModule::set_visible(boma, mouth_fire_r, false);
        }
    }
}

// #[skyline::hook(offset = 0x646fc0)]
// pub unsafe extern "C" fn something(fighter: *mut u64) -> u32 {
//     original!()(fighter)
// }

pub fn install() {
    skyline::install_hooks!(
        zard_set_flame,
        zard_set_hide_flame,
        // something
    );
}
