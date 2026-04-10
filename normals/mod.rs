use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*},

    smash::app::FighterAvailableFinal,

    skyline::nn::ro::LookupSymbol,
    std::convert::TryInto
};

use smash::app;
use super::*;
use crate::MARKED_COLORS;

pub const SITUATION_KIND: i32 = 0x16;
pub const PREV_STATUS_KIND: i32 = 0xA;
pub const STATUS_KIND: i32 = 0xB;
pub const STATUS_FRAME: i32 = 0xE;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;

pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME :i32 = 0x100000DC;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL :i32 = 0x100000DF;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_POWER_MULT :i32 = 0x110000DD;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_TIMER :i32 = 0x100000DD;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF :i32 = 0x100000DE;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_POTION_DRINK :i32 = 0x200000F0;
//pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POTION_TYPE :i32 = 0x100000DF;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_TAKEN :i32 = 0x110000DE;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_START :i32 = 0x110000DF;
//pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POTION_CD :i32 = 0x100000E0;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_STOP :i32 = 0x200000F1;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_DOWN_B_DISABLE_AIR :i32 = 0x200000F2;
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL :i32 = 0x200000F3;

pub const WEAPON_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL :i32 = 0x20000030;
pub const WEAPON_MEGUMIN_INSTANCE_WORK_ID_FLAG_EXPLOSION_AERIAL :i32 = 0x10000050;





unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 1.7, 90, 20, 0, 20, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 1.7, 90, 20, 0, 20, 3.0, 0.5, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 1.7, 90, 20, 0, 20, 3.0, 1.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 1.7, 90, 20, 0, 20, 3.0, 1.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.7, 270, 20, 0, 20, 3.5, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.6, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.7, 90, 20, 0, 20, 3.5, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 20.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 8.4, 361, 70, 0, 45, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 8.4, 361, 70, 0, 45, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 8.4, 361, 70, 0, 45, 3.5, 1.0, 0.0, 7.7, Some(1.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 0.85);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 4.3, 90, 80, 0, 42, 3.4, 1.0, 0.0, 4.0, Some(1.0), Some(0.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 4.3, 90, 80, 0, 42, 3.4, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 4.3, 90, 80, 0, 42, 3.4, 1.0, 0.0, 5.5, Some(1.0), Some(0.0), Some(9.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("kneer"), 5.1, 82, 65, 0, 90, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.2, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.1, 82, 65, 0, 90, 2.5, 0.0, 2.5, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.2, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}



unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 12.0, 30, 90, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 9.0, 30, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 7.0, 30, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 9.0, 30, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 7.0, 30, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 5.0, 30, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 7.0, 30, 90, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("hip"), 5.0, 30, 90, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 2.0, 30, 90, 0, 40, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.85);
    
}


unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("ball"), 25.4, 361, 80, 0, 45, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 12.4, 361, 65, 0, 35, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 12.4, 361, 65, 0, 35, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 4.4, 361, 25, 0, 35, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        
        let mut chargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        chargetime -= (25.4 * 9.0) as i32;
        if chargetime < 0 {chargetime = 0;}
        WorkModule::set_int(agent.module_accessor, chargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}




unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 1, Hash40::new("sword1"), 2.0, 90, 90, 90, 0, 3.5, 1.0, 0.0, 1.0, Some(1.0), Some(0.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 2.0, 90, 90, 90, 0, 3.5, 1.0, 0.0, 7.7, Some(1.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 3, 0, Hash40::new("armr"), 3.8, 361, 90, 0, 80, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ball"), 21.4, 85, 75, 0, 45, 7.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 4.4, 361, 25, 0, 35, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 4.4, 361, 25, 0, 35, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 4.4, 361, 25, 0, 35, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    
        let mut chargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        chargetime -= (21.4 * 9.0) as i32;
        if chargetime < 0 {chargetime = 0;}
        WorkModule::set_int(agent.module_accessor, chargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ball"), 10.7, 85, 75, 0, 45, 10.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.2, 361, 75, 0, 44, 7.5, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.2, 160, 70, 0, 40, 6.5, 0.0, 4.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 4.4, 361, 25, 0, 35, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("armr"), 4.4, 361, 25, 0, 35, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 4.4, 361, 25, 0, 35, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        DamageModule::add_damage(agent.module_accessor, 7.5, 0);

        let mut chargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        chargetime -= (16.2 * 9.0) as i32;
        if chargetime < 0 {chargetime = 0;}
        WorkModule::set_int(agent.module_accessor, chargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }
    /* wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    } */
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.2, 361, 75, 0, 44, 12.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}



unsafe extern "C" fn game_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 4.6, 361, 125, 0, 20, 3.8, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 4.6, 361, 125, 0, 20, 4.0, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("legl"), 4.6, 361, 60, 0, 20, 3.8, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneel"), 4.6, 361, 60, 0, 20, 4.0, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 4.6, 1, 125, 0, 20, 3.8, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 4.6, 1, 125, 0, 20, 4.0, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("legl"), 4.6, 1, 60, 0, 20, 3.8, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneel"), 4.6, 1, 60, 0, 20, 4.0, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


unsafe extern "C" fn game_attackairf(agent: &mut L2CAgentBase) {
    /* frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
    } */
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 4, 0, Hash40::new("sword1"), 3.8, 361, 90, 0, 80, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("sword1"), 3.8, 361, 90, 0, 80, 3.5, 1.0, 0.0, 7.7, Some(1.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 6, 0, Hash40::new("armr"), 3.8, 361, 90, 0, 80, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 4, 0, Hash40::new("sword1"), 6.5, 260, 60, 0, 24, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("sword1"), 6.5, 260, 60, 0, 24, 3.5, 1.0, 0.0, 7.7, Some(1.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 6, 0, Hash40::new("armr"), 6.5, 260, 60, 0, 24, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ball"), 13.7, 270, 82, 0, 20, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("ball"), 13.7, 361, 82, 0, 20, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        
        let mut chargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        chargetime -= (16.7 * 9.0) as i32;
        if chargetime < 0 {chargetime = 0;}
        WorkModule::set_int(agent.module_accessor, chargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("ball"), 8.4, 270, 82, 0, 20, 6.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("ball"), 8.4, 361, 82, 0, 20, 6.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 4, 0, Hash40::new("sword1"), 1.7, 361, 40, 0, 30, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("sword1"), 1.7, 361, 40, 0, 30, 3.5, 1.0, 0.0, 7.7, Some(1.0), Some(0.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 6, 0, Hash40::new("armr"), 1.7, 361, 40, 0, 30, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn game_landingairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ball"), 7.2, 361, 75, 0, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 3.6, 270, 87, 0, 20, 3.3, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}


unsafe extern "C" fn game_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("ball"), 16.1, 361, 75, 0, 40, 3.7, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("ball"), 8.1, 361, 75, 0, 40, 8.1, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 6.5, 25, 65, 0, 70, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 6.5, 25, 65, 0, 70, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("armr"), 6.5, 25, 65, 0, 70, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        
        let mut chargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        chargetime -= (16.1 * 9.0) as i32;
        if chargetime < 0 {chargetime = 0;}
        WorkModule::set_int(agent.module_accessor, chargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 2.5, 361, 59, 0, 35, 3.5, 1.0, 0.0, 2.5, Some(1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 2.5, 361, 59, 0, 35, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("armr"), 2.5, 361, 59, 0, 35, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 367, 0, 100, 30, 4.2, 0.0, 20.0, -9.0, Some(0.0), Some(20.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.04, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 90, 90, 0, 37, 4.2, 0.0, 20.0, 4.5, Some(0.0), Some(17.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 90, 90, 0, 37, 4.2, 0.0, 20.0, 9.0, Some(0.0), Some(17.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    for i in 0..32 {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.08, z: 0.0});
            if KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.15 {
                KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.06, z: 0.0});
            }
        }
        wait(agent.lua_state_agent, 1.0);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.08, z: 0.0});
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.6, 85, 170, 0, 45, 4.6, 0.0, 20.0, -5.0, Some(0.0), Some(20.0), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 90, 90, 0, 37, 4.2, 0.0, 20.0, 4.5, Some(0.0), Some(17.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        //macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 90, 90, 0, 37, 4.2, 0.0, 20.0, 9.0, Some(0.0), Some(17.5), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0.3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
        KineticModule::add_speed(agent.module_accessor, &Vector3f{ x: 0.0, y: 0.12, z: 0.0});
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("legl"), 3.6, 270, 87, 0, 20, 3.3, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("legl"), 3.6, 270, 87, 0, 20, 3.3, 3.2, 2.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneel"), 3.6, 270, 87, 0, 20, 3.3, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("kneel"), 3.6, 270, 87, 0, 20, 3.3, 6.2, 0.9, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 2.0, false);
        if KineticModule::get_sum_speed_y(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.1 {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}




unsafe extern "C" fn eff_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

unsafe extern "C" fn eff_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}
unsafe extern "C" fn eff_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -12.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}
unsafe extern "C" fn eff_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.65, 0, 0, 4, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.4, 0, 0, 5, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_sliding_smoke"), Hash40::new("top"), 6, 0, -3, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.36, 0, 0, 5, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.28, 0, 0, 5, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, -3, 0, 0, 0, 0.28, 0, 0, 5, 0, 0, 0, false);
    }
    /* frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 7, 0, -3, 0, 0, 0, 0.35, 0, 0, 5, 0, 0, 0, false);
    } */
}

unsafe extern "C" fn eff_attackdash(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn eff_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_c"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}
unsafe extern "C" fn eff_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn eff_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn eff_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 12, 3, 0, 100, 90, 0.8, true, 0.75);
        macros::LAST_EFFECT_SET_RATE(agent, 0.8);
    }
}
unsafe extern "C" fn eff_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}
unsafe extern "C" fn eff_landingairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn eff_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("ball"), 0.0, 0.0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}
unsafe extern "C" fn eff_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}
unsafe extern "C" fn eff_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_line"), Hash40::new("top"), -1, 6, -1, 90, 0, 0, 0.8, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.6);
    }
}

unsafe extern "C" fn expression_attackairf(agent: &mut L2CAgentBase) {
    /* if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    } */
}



/* AFTER_IMAGE4_ON_arg29(fighter,
    Hash40::new("tex_roy_sword1"), Hash40::new("tex_roy_sword2"), /*effect texture name*/
    7/*IDK*/,
    Hash40::new("sword1")/*Start Bone*/,
    -0.5, 0.0, -5.0, /*Start Bone Offsets X,Y,Z (Coincidentally seems to be the same blender bone XYZ?)*/
    Hash40::new("sword1")/*End Bone*/,
    -0.5, -0.0, 14.5, /*End Bone Offsets X,Y,Z*/
    true,/*IDK*/
    Hash40::new("roy_sword"), Hash40::new("h_exo_righthandsword"),
    0, 0, 0, /*Glow Offset XYZ*/
    0, 0, 0, /*Glow Rotation XYZ*/
    1 /*Size*/,
    0, *EFFECT_AXIS_X, /*IDK*/
    0, *TRAIL_BLEND_ALPHA, /*IDK*/
    101, *TRAIL_CULL_NONE, /*IDK*/
    1.3, 0.2 /*IDK*/);
     */   



unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_smash"));
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_brave_smash_s01"));
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_s03_01"));
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l06_02"));
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_ll"));
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_landing_01"));
    }
}


unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_smash_h01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_L02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_attack09"));
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}


unsafe extern "C" fn sound_attacklw4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_attack08"));
        macros::PLAY_SE(agent, Hash40::new("se_brave_smash_l01"));
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l05_02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_smash_l02"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_attackair_b01"));
    }
}


unsafe extern "C" fn sound_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_attackair_f01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_brave_rnd_attack02"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}

unsafe extern "C" fn sound_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_attackair_b01"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_brave_rnd_attack02"));
    }
}




unsafe extern "C" fn megumin_attacks3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.status_AttackS3Common();
    if ControlModule::get_stick_y(fighter.module_accessor) > 0.4 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s3_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if ControlModule::get_stick_y(fighter.module_accessor) < -0.4 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s3_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_s3_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_attacks3_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_attacks3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    } else {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }
    return 0.into();
}




unsafe extern "C" fn megumin_attacks4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS4();
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_attacks4_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_attacks4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
        }
    } 
    return 0.into();
}









unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 7.0, 7.5, Some(0.0), Some(7.0), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G_d);
        macros::CATCH(agent, 1, Hash40::new("top"), 1.65, 0.0, 7.0, 5.85, Some(0.0), Some(7.0), Some(14.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}







unsafe extern "C" fn empty_acmd(agent: &mut L2CAgentBase) {
}





unsafe extern "C" fn game_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.0, 36, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 14, 3);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 7.5, y: 1.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 65, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
        macros::CHECK_FINISH_CAMERA(agent, 11, 6);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 9.5, y: 0.5, z: 0.0});
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 89, 80, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 0, 31);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.6);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 2.0, y: 8.5, z: 0.0});
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}


unsafe extern "C" fn game_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 0, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 1, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        //AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        //AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("throw"), 1.0, 270, 50, 0, 30, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 4.0, 270, 10, 10, 10, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_BODY);
        AttackModule::set_attack_level(agent.module_accessor, 0, 0 as u8);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 20.0, false);
        //macros::FT_MOTION_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}



unsafe extern "C" fn effect_throwlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("throw"), 0.0, 0.0, 0.0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_hit_sweat"), Hash40::new("throw"), 0.0, 0.0, 0.0, 75, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn effect_throwhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("throw"), 0.0, 0.0, 0.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("throw"), 0.0, 0.0, 0.0, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_throwhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_throw_02"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_brave_rnd_attack02"));
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_l"));
    }
}



unsafe extern "C" fn expression_appeallwl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 52.0);
    if macros::is_excute(agent) {
        //ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn effect_appealhil(agent: &mut L2CAgentBase) {
    /* frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_tension"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 2, 0.15, 2.2, 0);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_FRAME(agent, 32, 2, 0.15, 2.2, 1);
    }
    wait(agent.lua_state_agent, 32.0);
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::BURN_COLOR(agent, 2, 0.15, 2.2, 1);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_FRAME(agent, 12, 2, 0.15, 2.2, 0);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    } */
}

unsafe extern "C" fn sound_appeallwl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_l03"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_step_left_ll02"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_l03"));
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_step_right_ll03"));
        macros::PLAY_SE(agent, Hash40::new("se_brave_rise"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_l03"));
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_l03"));
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_squat"));
    }
    frame(agent.lua_state_agent, 80.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_l03"));
    }
    frame(agent.lua_state_agent, 82.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_step_left_m03"));
    }
    frame(agent.lua_state_agent, 86.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_step_left_m02"));
    }
    frame(agent.lua_state_agent, 100.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_l03"));
    }
}

unsafe extern "C" fn sound_appealsl(agent: &mut L2CAgentBase) {
    /* frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_appeal_s01"));
    } */
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_appeal02"));
    }
}




unsafe extern "C" fn game_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 45, 20, 0, 90, 5.0, 0.0, 5.0, -3.5, Some(0.0), Some(5.0), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_cliffattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 6, 3, -5, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
        //macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}


unsafe extern "C" fn game_downattackd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, -16.0, Some(0.0), Some(5.0), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, 16.5, Some(0.0), Some(5.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_downattackd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn game_downattacku(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, -16.5, Some(0.0), Some(5.0), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 48, 48, 0, 80, 5.0, 0.0, 5.0, 16.5, Some(0.0), Some(5.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_downattacku(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

unsafe extern "C" fn game_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, 20.0, Some(0.0), Some(5.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 50, 0, 60, 5.0, 0.0, 5.0, -17.0, Some(0.0), Some(5.0), Some(-5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_slipattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_brave_sword1"), Hash40::new("tex_brave_sword2"), 5, Hash40::new("sword1"), 1.5, 0, 0, Hash40::new("sword1"), 14.4, 0, 0, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
        //macros::EFFECT_FOLLOW_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, true);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 3, 0, 0, 0, 180, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_megumin_sword1"), Hash40::new("tex_megumin_sword2"), 5, Hash40::new("sword1"), 0, 0, -2.0, Hash40::new("sword1"), 0, 0, 15.4, true, Hash40::new("null"), Hash40::new("sword1"), 0, 0, 0, 0, 0, -90, 1, 0, *EFFECT_AXIS_Z, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.1);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_OFF_KIND_WORK(agent, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}


unsafe extern "C" fn sound_furafura(agent: &mut L2CAgentBase) {
    loop {
        frame(agent.lua_state_agent, 1.0);
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FURAFURA_VOICE_FINAL) {
            if macros::is_excute(agent) {
                macros::PLAY_STATUS(agent, Hash40::new("vc_brave_furafura"));
                macros::PLAY_SE(agent, Hash40::new("se_brave_special_h04"));
            }
        }
        
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}



unsafe extern "C" fn sound_furafuraend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_brave_appear01"));
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_s04_01"));
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_kick_hit_m"));
    }
}

unsafe extern "C" fn effect_furafurastartd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("gato"), 0, 0, 0, 0, 0, 0, 0.34, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_furafurastartd(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FURAFURA_VOICE_FINAL) {
        if macros::is_excute(agent) {
            macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_common_dizzy"), 10);
        }
        else {
            frame(agent.lua_state_agent, 10.0);
            if macros::is_excute(agent) {
                macros::PLAY_STATUS(agent, Hash40::new("se_common_dizzy"));
            }
            frame(agent.lua_state_agent, 30.0);
            if macros::is_excute(agent) {
                macros::PLAY_SE(agent, Hash40::new("se_common_kick_hit_s"));
            }
        }
    }
}




unsafe extern "C" fn sound_entryl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_s04_02"));
    }
}



unsafe extern "C" fn effect_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("ezkudu"), 0, 0, 0, 0, 0, -90, 3.1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_h01_02"));
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_ll"));
    }
}

unsafe extern "C" fn sound_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_h01_01"));
    }
}

unsafe extern "C" fn sound_win3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_h03_02"));
    }
}

unsafe extern "C" fn sound_win3wait(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_s03_02"));
    }
}

unsafe extern "C" fn megumin_squat_empty(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}



unsafe extern "C" fn megumin_sleep_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_DamageSongEnd();
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_sleep_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_sleep_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
    }
    return 0.into();
}







pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe {
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }

    Agent::new("brave")
    .set_costume(costume.to_vec())
        .game_acmd("game_attack11", game_attack11, Default)
        .game_acmd("game_attacks3", game_attacks3, Default)
        .game_acmd("game_attacks3hi", game_attacks3, Default)
        .game_acmd("game_attacks3lw", game_attacks3, Default)
        .game_acmd("game_attackhi3", game_attackhi3, Default)
        .game_acmd("game_attacklw3", game_attacklw3, Default)
        .game_acmd("game_attackdash", game_attackdash, Default)
        .game_acmd("game_attacks4", game_attacks4, Default)
        .game_acmd("game_attackhi4", game_attackhi4, Default)
        .game_acmd("game_attacklw4", game_attacklw4, Default)
        .game_acmd("game_attackairn", game_attackairn, Default)
        .game_acmd("game_attackairf", game_attackairf, Default)
        .game_acmd("game_landingairf", game_landingairf, Default)
        .game_acmd("game_attackairb", game_attackairb, Default)
        .game_acmd("game_attackairhi", game_attackairhi, Default)
        .game_acmd("game_attackairlw", game_attackairlw, Default)

        .effect_acmd("effect_attack11", eff_attack11, Default)
        .effect_acmd("effect_attacks3", eff_attacks3, Default)
        .effect_acmd("effect_attackhi3", eff_attackhi3, Default)
        .effect_acmd("effect_attacklw3", eff_attacklw3, Default)
        .effect_acmd("effect_attackdash", eff_attackdash, Default)
        .effect_acmd("effect_attacks4", eff_attacks4, Default)
        .effect_acmd("effect_attackhi4", eff_attackhi4, Default)
        .effect_acmd("effect_attacklw4", eff_attacklw4, Default)
        .effect_acmd("effect_attackairn", eff_attackairn, Default)
        .effect_acmd("effect_attackairf", eff_attackairf, Default)
        .effect_acmd("effect_landingairf", eff_landingairf, Default)
        .effect_acmd("effect_attackairb", eff_attackairb, Default)
        .effect_acmd("effect_attackairhi", eff_attackairhi, Default)
        .effect_acmd("effect_attackairlw", eff_attackairlw, Default)
        .sound_acmd("sound_attacks4", sound_attacks4, Default)
        .sound_acmd("sound_attackhi4", sound_attackhi4, Default)
        .sound_acmd("sound_attackairf", sound_attackairf, Default)
        .sound_acmd("sound_attackairb", sound_attackairb, Default)
        .expression_acmd("expression_attackairf", expression_attackairf, Default)

        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, megumin_attacks3_main)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S4, megumin_attacks4_main)
        //.status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, megumin_attacks4_end)

        .game_acmd("game_wait1", empty_acmd, Default)
        .game_acmd("game_wait2", empty_acmd, Default)
        .game_acmd("game_wait3", empty_acmd, Default)
        .game_acmd("game_squatwait1", empty_acmd, Default)
        .game_acmd("game_walkslow", empty_acmd, Default)
        .game_acmd("game_walkmiddle", empty_acmd, Default)
        .game_acmd("game_walkfast", empty_acmd, Default)
        .status(Init, *FIGHTER_STATUS_KIND_SQUAT_WAIT, megumin_squat_empty)
        .status(Exec, *FIGHTER_STATUS_KIND_SQUAT_WAIT, megumin_squat_empty)
        .status(ExecStop, *FIGHTER_STATUS_KIND_SQUAT_WAIT, megumin_squat_empty)

        .game_acmd("game_catch", game_catch, Default)
        //.effect_acmd("effect_catch", effect_catch, Default)

        .game_acmd("game_throwf", game_throwf, Default)
        .game_acmd("game_throwb", game_throwb, Default)
        .game_acmd("game_throwlw", game_throwlw, Default)
        .game_acmd("game_throwhi", game_throwhi, Default)
        
        .effect_acmd("effect_throwlw", effect_throwlw, Default)
        .effect_acmd("effect_throwhi", effect_throwhi, Default)
        .sound_acmd("sound_throwhi", sound_throwhi, Default)
        
        .expression_acmd("expression_appeallwl", expression_appeallwl, Default)
        .expression_acmd("expression_appeallwr", expression_appeallwl, Default)
        .effect_acmd("effect_appeallwl", effect_appealhil, Default)
        .effect_acmd("effect_appeallwr", effect_appealhil, Default)
        .effect_acmd("effect_appealhil", effect_appealhil, Default)
        .effect_acmd("effect_appealhir", effect_appealhil, Default)
        .effect_acmd("effect_appealsl", effect_appealhil, Default)
        .effect_acmd("effect_appealsr", effect_appealhil, Default)
        .sound_acmd("sound_appealsl", sound_appealsl, Default)
        .sound_acmd("sound_appealsr", sound_appealsl, Default)
        .sound_acmd("sound_appeallwl", sound_appeallwl, Default)
        .sound_acmd("sound_appeallwr", sound_appeallwl, Default)
        
        /* .sound_acmd("sound_entryl", sound_entry, Default)
        .sound_acmd("sound_entryr", sound_entry, Default) */
        
        .game_acmd("game_cliffattack", game_cliffattack, Default)
        .effect_acmd("effect_cliffattack", effect_cliffattack, Default)
        .game_acmd("game_downattackd", game_downattackd, Default)
        .effect_acmd("effect_downattackd", effect_downattackd, Default)
        .game_acmd("game_downattacku", game_downattacku, Default)
        .effect_acmd("effect_downattacku", effect_downattacku, Default)
        .game_acmd("game_slipattack", game_slipattack, Default)
        .effect_acmd("effect_slipattack", effect_slipattack, Default)
        
        .sound_acmd("sound_furafura", sound_furafura, Default)
        .sound_acmd("sound_furafuraend", sound_furafuraend, Default)
        .effect_acmd("effect_furafurastartd", effect_furafurastartd, Default)
        .effect_acmd("effect_furafurastartu", effect_furafurastartd, Default)
        .sound_acmd("sound_furafurastartd", sound_furafurastartd, Default)
        .sound_acmd("sound_furafurastartu", sound_furafurastartd, Default)

        .sound_acmd("sound_entryl", sound_entryl, Default)
        .sound_acmd("sound_entryr", sound_entryl, Default)

        .sound_acmd("sound_win3", sound_win1, Default)
        .sound_acmd("sound_win1", sound_win3, Default)
        .sound_acmd("sound_win3wait", sound_win3wait, Default)
        .effect_acmd("effect_win2", effect_win2, Default)
        .sound_acmd("sound_win2", sound_win2, Default)

        //.status(End, *FIGHTER_STATUS_KIND_SLEEP_END, megumin_sleep_end)
        //.status(End, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, megumin_sleep_end)
        .status(Main, *FIGHTER_STATUS_KIND_DAMAGE_SONG_END, megumin_sleep_main)

        //.on_start(on_start)

        .install();
}