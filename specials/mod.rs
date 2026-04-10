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
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR :i32 = 0x200000F4;

pub const WEAPON_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL :i32 = 0x20000030;
pub const WEAPON_MEGUMIN_INSTANCE_WORK_ID_FLAG_EXPLOSION_AERIAL :i32 = 0x10000050;






unsafe extern "C" fn megumin_upb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_STOP);
        let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0,
            2.0
        );
        sv_kinetic_energy!(
            set_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_STOP);
        let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_STOP);
        let speed_x = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(
            set_speed,
            agent,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_x,
            0.0
        );
    }
}

unsafe extern "C" fn megumin_upb_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_h02_01"));
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_h02_02"));
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_h03_01"));
    }
}






unsafe extern "C" fn megumin_upb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    //fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);

    //StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    //StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);

    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, 0, 0, 0);
    //fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_hi").into());
    0.into()
}

unsafe extern "C" fn megumin_upb_init(fighter: &mut L2CFighterCommon) -> L2CValue {

    fighter.clear_lua_stack();
    lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    0.into()
}

unsafe extern "C" fn megumin_upb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);

    //if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR);
    //}
    
    //fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_upb_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_upb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_STOP) {

        fighter.clear_lua_stack();
        lua_args!(fighter,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY,*ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        /* fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent); */

        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_STOP);
    }

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    //WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_DOWN_B_DISABLE_AIR) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if fighter.global_table[STATUS_FRAME].get_f32() > 1.0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    }
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
    }

    fighter.sub_transition_group_check_air_special();
    fighter.sub_transition_group_check_air_attack();
    fighter.sub_transition_group_check_air_escape();


    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);

    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * 0.7
    );
    sv_kinetic_energy!(
        set_accel_x_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_accel_x_mul * 0.6
    );


    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR);

    if MotionModule::is_end(fighter.module_accessor) {
        //WorkModule::on_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR);
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    return 0.into();
}









unsafe extern "C" fn megumin_neutralb0(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //WorkModule::set_int(agent.module_accessor, 0, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        macros::ATTACK(agent, 0, 0, Hash40::new("explosion"), 0.0, 270, 10, 10, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("explosion"), 0.0, 270, 10, 10, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.4, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("explosion"), 0.0, 270, 10, 10, 0, 19.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.15, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn megumin_neutralb1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
        WorkModule::set_int(agent.module_accessor, 1, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
        let mut newchargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME) - (33 * 36);
        if newchargetime < 0 {newchargetime = 0;}
        WorkModule::set_int(agent.module_accessor, newchargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        //damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 65.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn megumin_neutralb2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 15);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
        WorkModule::set_int(agent.module_accessor, 2, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
        let mut newchargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME) - (66 * 36);
        if newchargetime < 0 {newchargetime = 0;}
        WorkModule::set_int(agent.module_accessor, newchargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 91.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn megumin_neutralb3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
    }
    /* frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
    } */
    frame(agent.lua_state_agent, 145.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_START_FLAG_STRENGTH);
        WorkModule::set_int(agent.module_accessor, 3, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_EXPLOSION, false, -1);
        WorkModule::set_int(agent.module_accessor, 0, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    frame(agent.lua_state_agent, 146.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn sound_neutralb0(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_n01"));
    }
}

unsafe extern "C" fn sound_neutralb1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_n02_01"));
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 64.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_n05"));
    }
}

unsafe extern "C" fn sound_neutralb2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_n03_01"));
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 90.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l06"));
    }
}

unsafe extern "C" fn sound_neutralb3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_n04_01"));
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 58.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 68.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_l02"));
    }
    frame(agent.lua_state_agent, 145.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_n06"));
    }
}


unsafe extern "C" fn megumin_neutralb0_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_damage_fire_fly"), Hash40::new("explosion"), 0.0, 0.0, 0.0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn megumin_neutralb1_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("explosion"), 0.0, 0.0, 0.0, 0, 0, 180, 2.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn megumin_neutralb2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("explosion"), 0.0, 0.0, 0.0, 0, 0, 0, 2.5, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn megumin_neutralb3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_d"), Hash40::new("explosion"), 0.0, 0.0, 0.0, 0, 0, 0, 2.6, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn megumin_neutralb_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_bomb_m"));
    }
}

unsafe extern "C" fn megumin_neutralb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32, 0);
    0.into()
}

unsafe extern "C" fn megumin_neutralb_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    0.into()
}

unsafe extern "C" fn megumin_neutralb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);

    let chargetime = WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);

    if chargetime >= 3600 {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n3").into(), Hash40::new("special_air_n3").into(), false.into());
    } else if chargetime >= 2400 {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n2").into(), Hash40::new("special_air_n2").into(), false.into());
    } else if chargetime >= 1200 {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n1").into(), Hash40::new("special_air_n1").into(), false.into());
    } else {
        fighter.sub_change_motion_by_situation(Hash40::new("special_n_fail").into(), Hash40::new("special_air_n_fail").into(), false.into());
    }

    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    //KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
    
    //fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_neutralb_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_neutralb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();

    let chargetime = WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    
    
    if !MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.set_situation(SITUATION_KIND_AIR.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n_fail") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_fail"), -1.0, 1.0, 0.0, true, true);
                } 
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n1") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n1"), -1.0, 1.0, 0.0, true, true);
                } 
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n2") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n2"), -1.0, 1.0, 0.0, true, true);
                } 
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n3") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n3"), -1.0, 1.0, 0.0, true, true);
                } 
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            } else {
                fighter.set_situation(SITUATION_KIND_GROUND.into());
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n_fail") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_fail"), -1.0, 1.0, 0.0, true, true);
                } 
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n1") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n1"), -1.0, 1.0, 0.0, true, true);
                } 
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n2") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n2"), -1.0, 1.0, 0.0, true, true);
                } 
                else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n3") {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n3"), -1.0, 1.0, 0.0, true, true);
                } 
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            }
        }
    } else {
        if situation_kind != *SITUATION_KIND_GROUND {
            //if chargetime >= 1200 && chargetime < 3600 {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n_fail") {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            }
        }
        else {
            //if chargetime >= 2400 && chargetime < 3600 {
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n_fail") {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            } else {
                fighter.change_status(FIGHTER_STATUS_KIND_DOWN_WAIT.into(), false.into());
            }
        }
    }

    if (fighter.global_table[0x16] != *SITUATION_KIND_GROUND) {
        let mut height = 1.1;
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n1") {
            height = 1.3
        } 
        else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n2") {
            height = 2.0
        } 
        else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_n3") {
            height = 3.0
        } 

        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_NEUTRAL_B_AIR_STALL) {
            let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            notify_event_msc_cmd!(
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed_y * 0.87
            );
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            notify_event_msc_cmd!(
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE) {
            notify_event_msc_cmd!(
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                height
            );
            sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
            notify_event_msc_cmd!(
                fighter,
                *FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                accel_y * -1.0
            );
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
        }

    }
    return 0.into();
}







unsafe extern "C" fn neutralb_proj_explode1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //macros::ATTACK(agent, 3, 0, Hash40::new("top"), 0.0, 366, 100, 100, 0, 35.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        //macros::ATTACK(agent, 4, 0, Hash40::new("top"), 0.0, 366, 100, 50, 0, 35.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 3, 1, Hash40::new("top"), 0.0, 366, 50, 35, 0, 30.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 1, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::AREA_WIND_2ND_RAD(agent, 0, 0.5, 0.01, 1000, 1, 0, 0, 40);
    ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 14, true, *BATTLE_OBJECT_ID_INVALID as u32);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 26.0, 65, 50, 0, 80, 13.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -13, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 21.0, 60, 50, 0, 80, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.7, 361, 70, 0, 35, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 361, 65, 0, 30, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 21.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.1, 361, 70, 0, 35, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.6, 361, 65, 0, 30, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 21.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::AREA_WIND_2ND_RAD(agent, 0, 1, 0.1, 1000, 1, 0, 0, 40);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.3, 361, 70, 0, 35, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.3, 361, 65, 0, 30, 15.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 21.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 1.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn neutralb_proj_explode2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 1, Hash40::new("top"), 0.0, 366, 80, 50, 0, 35.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 1, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::AREA_WIND_2ND_RAD(agent, 0, 0.5, 0.01, 1000, 1, 0, 0, 40);
    ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 14, true, *BATTLE_OBJECT_ID_INVALID as u32);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 26.0, 65, 50, 0, 80, 13.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -13, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 21.0, 60, 50, 0, 80, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 29.0, 361, 80, 0, 45, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.95, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.7, 361, 70, 0, 35, 19.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 361, 65, 0, 30, 26.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 19.6, 361, 80, 0, 45, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.95, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 11.1, 361, 70, 0, 35, 19.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 4.6, 361, 65, 0, 30, 26.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::AREA_WIND_2ND_RAD(agent, 0, 1, 0.1, 1000, 1, 0, 0, 40);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.3, 361, 80, 0, 45, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.95, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.3, 361, 70, 0, 35, 19.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.3, 361, 65, 0, 30, 26.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn neutralb_proj_explode3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 1, Hash40::new("top"), 0.0, 366, 100, 100, 0, 45.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 1, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::AREA_WIND_2ND_RAD(agent, 0, 0.5, 0.01, 1000, 1, 0, 0, 40);
    ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattacks"), 14, true, *BATTLE_OBJECT_ID_INVALID as u32);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        //macros::ATTACK(agent, 0, 0, Hash40::new("top"), 26.0, 65, 50, 0, 80, 13.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -13, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 21.0, 60, 50, 0, 80, 18.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.5), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -10, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 50.0, 361, 70, 0, 45, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 29.0, 361, 80, 0, 45, 21.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 16.7, 361, 70, 0, 35, 28.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 33.3, 361, 70, 0, 45, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 19.6, 361, 80, 0, 45, 21.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.1, 361, 70, 0, 35, 28.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::AREA_WIND_2ND_RAD(agent, 0, 1, 0.1, 1000, 1, 0, 0, 40);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 17.6, 361, 70, 0, 45, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.3, 361, 80, 0, 45, 21.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.3, 361, 70, 0, 35, 28.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}



unsafe extern "C" fn eff_explode1(agent: &mut L2CAgentBase) {
    //frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_hit_fire"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 2.1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("brave_explosion2_omen"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("brave_explosion2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("brave_explosion2_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn eff_explode2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("brave_explosion2_omen"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("brave_explosion2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("brave_explosion2_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn eff_explode3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("brave_explosion2_omen"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("brave_explosion2"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("brave_explosion2_flash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

pub unsafe extern "C" fn neutralb_proj_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32, //*GROUND_CORRECT_KIND_KEEP as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn neutralb_proj_main(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_lr = PostureModule::lr(owner);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move2"), 0.0, 1.0, false, 0.0, false, false);

    let level = WorkModule::get_int(owner, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL);
    
    
    let mut offset = Vector3f{x:0.0,y:0.0,z:0.0};
    
    let mut owner_bone_pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40::new("magikacirculo1"), &mut owner_bone_pos);
    let newPos = Vector3f{
        x: PostureModule::pos_x(owner) + owner_bone_pos.x + (offset.x*lr), 
        y: PostureModule::pos_y(owner) + owner_bone_pos.y + offset.y, 
        z: 0.0
    };
    PostureModule::set_pos(weapon.module_accessor, &newPos); 



    if StatusModule::situation_kind(owner) == SITUATION_KIND_AIR {
        WorkModule::on_flag(weapon.module_accessor, WEAPON_MEGUMIN_INSTANCE_WORK_ID_FLAG_EXPLOSION_AERIAL);
    } else {
        WorkModule::off_flag(weapon.module_accessor, WEAPON_MEGUMIN_INSTANCE_WORK_ID_FLAG_EXPLOSION_AERIAL);
    }

    let life = 72;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    
    if StopModule::is_stop(weapon.module_accessor) {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }

    weapon.fastshift(L2CValue::Ptr(neutralb_proj_main_loop as *const () as _))
}

unsafe extern "C" fn neutralb_proj_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let level = WorkModule::get_int(owner, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL) as f32;

    //let level = WorkModule::get_int(weapon.module_accessor, WEAPON_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL) as f32;
    let mut angle: f32 = 0.0;
    if WorkModule::is_flag(weapon.module_accessor, WEAPON_MEGUMIN_INSTANCE_WORK_ID_FLAG_EXPLOSION_AERIAL) {
        angle = 315.0;
    }

    let lr = PostureModule::lr(weapon.module_accessor);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let num_speed = 2.0 + level;
    
    let speed_x = (angle.to_radians()).cos() * num_speed;
    let speed_y = (angle.to_radians()).sin() * num_speed;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );

    println!("angle {}", angle);
    println!("speed x {}", speed_x);
    println!("speed y {}", speed_y);

    if life < 30 || GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) || AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ALL) {
        //smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        WorkModule::set_int(weapon.module_accessor, 30, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        StatusModule::change_status_force(weapon.module_accessor, *WEAPON_BRAVE_EXPLOSION_STATUS_KIND_EXPLODE2, false);
    }

    0.into()
}



unsafe extern "C" fn game_move2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.6, 361, 40, 0, 25, 5.5, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::enable_safe_pos(agent.module_accessor);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 4.0, false);
    }
}

unsafe extern "C" fn effect_move2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_explosion2_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, false);
    }
}


pub unsafe extern "C" fn neutralb_proj_explode_main(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let level = WorkModule::get_int(owner, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL);
    
    if level == 1 {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if level == 2 {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("move1"), 0.0, 1.0, false, 0.0, false, false);
    }
    if level == 3 {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode1"), 0.0, 1.0, false, 0.0, false, false);
    }

    weapon.fastshift(L2CValue::Ptr(neutralb_proj_explode_main_loop as *const () as _))
}

unsafe extern "C" fn neutralb_proj_explode_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    return 0.into();
}














unsafe extern "C" fn megumin_sideb1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 8.0, 6.0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
}

unsafe extern "C" fn sound_sideb1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_s01"));
        macros::PLAY_SE(agent, Hash40::new("se_brave_appear02"));
    }
}



unsafe extern "C" fn megumin_sideb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    //fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_START_TURN as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn megumin_sideb_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    0.into()
}

unsafe extern "C" fn megumin_sideb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);

    //KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_s").into());
    fighter.sub_change_motion_by_situation(Hash40::new("special_s1").into(), Hash40::new("special_air_s1").into(), false.into());
    
    //fighter.sub_change_motion_by_situation(Hash40::new("special_hi_start").into(), Hash40::new("special_air_hi_start").into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_sideb_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_sideb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if !MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s1"), -1.0, 1.0, 0.0, true, true);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        } else {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s1"), -1.0, 1.0, 0.0, true, true);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
    }
    } else {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    return 0.into();
}



unsafe extern "C" fn side_b_fireball_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 130, 100, 30, 0, 6.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, -10, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_specialn3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("brave_fire3_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}



unsafe extern "C" fn side_b_fireball_burst(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 115, 100, 50, 0, 8.5, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 10.0, 56, 70, 0, 60, 17.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 361, 80, 0, 80, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 70, 0, 50, 17.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn effect_burstl(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("brave_fire3_burst"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_burstl(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_special_n06"));
    }
}


pub unsafe extern "C" fn megumin_sideb_bullet_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let lr = PostureModule::lr(owner);
    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(owner,Hash40::new("cabesa"),&mut vec,false);
    PostureModule::set_pos(weapon.module_accessor, &vec);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        1.5*lr,
        0
    );


    0.into()
}














unsafe extern "C" fn megumin_downb1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 2.3);
        //motion rate rand stuff
    }
}

unsafe extern "C" fn megumin_downb1_air(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        //motion rate rand stuff
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_FLASH, false, -1);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FT_MOTION_RATE(agent, 0.4);
    }
}


unsafe extern "C" fn game_flashhit1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 45, 80, 0, 40, 10.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}

pub unsafe extern "C" fn downb_proj_main(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let hip_pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(owner, Hash40::new("hip"), hip_pos, false);
    
    PostureModule::set_pos(weapon.module_accessor, &(Vector3f {x: hip_pos.x, y: hip_pos.y, z: hip_pos.z}));

    StatusModule::change_status_force(weapon.module_accessor, *WEAPON_BRAVE_FLASH_STATUS_KIND_HIT1, false);
    return 0.into();
}


pub unsafe extern "C" fn downb_proj_explode_main(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let owner = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

    let life = 3;
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("flash_hit1"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(downb_proj_explode_main_loop as *const () as _))
}

unsafe extern "C" fn downb_proj_explode_main_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    if life < 0 {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }

    return 0.into();
}



unsafe extern "C" fn megumin_downb1_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
}

unsafe extern "C" fn megumin_downb1_air_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_bomb_a"), Hash40::new("hip"), 0.0, 0.0, 0.0, 0, 0, 0, 1.05, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn megumin_downb2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        let mut chargetime = WorkModule::get_int(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
        chargetime += (10 * 36) as i32;
        //if chargetime < 0 {chargetime = 0;}
        WorkModule::set_int(agent.module_accessor, chargetime, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);

        WorkModule::on_flag(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_POTION_DRINK);
    }
    frame(agent.lua_state_agent, 2.0);
    wait(agent.lua_state_agent, 1.0);
}

unsafe extern "C" fn megumin_downb2_sound(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_special_l01"));
    }
}

unsafe extern "C" fn sound_kamikaze(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_brave_missfoot01"));
    }
}


unsafe extern "C" fn megumin_downb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    //fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_n").into());
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, 0, *FIGHTER_STATUS_ATTR_START_TURN as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn megumin_downb_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0,
        0.0
    );
    0.into()
}

unsafe extern "C" fn megumin_downb_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_lw").into());
    fighter.sub_change_motion_by_situation(Hash40::new("special_lw_start").into(), Hash40::new("special_air_lw_start").into(), false.into());

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_DOWN_B_DISABLE_AIR);
    }


    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_downb_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_downb_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) && (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_start") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_start")) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_start") {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_cancel"), 0.0, 1.0, false, 0.0, false, false);
        } else if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_start") {
            if situation_kind != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_start") && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_start") && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw_cancel") /* || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw_cancel") */ {
        if !MotionModule::is_end(fighter.module_accessor) && MotionModule::frame(fighter.module_accessor) < 10.0 {
            if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_POTION_DRINK) {
                let roll1 = sv_math::rand(hash40("brave"), 3) as i32; //0-2
                let roll2 = sv_math::rand(hash40("brave"), 101) as f32; //0-100

                println!("roll1 {}", roll1);
                println!("roll2 {}", roll2);

                if roll1 == 0 { //power
                    let mut amount = 1.0;
                    if roll2 <= 30.0 { amount = 1.25}
                    else if roll2 <= 40.0 { amount = 1.2}
                    else if roll2 <= 45.0 { amount = 1.15}
                    else if roll2 <= 50.0 { amount = 1.1}
                    else if roll2 <= 55.0 { amount = 1.05}
                    else if roll2 <= 60.0 { amount = 1.3}
                    else if roll2 <= 65.0 { amount = 1.35}
                    else if roll2 <= 70.0 { amount = 1.5}

                    else if roll2 <= 82.0 { amount = 0.8}
                    else if roll2 <= 85.0 { amount = 0.75}
                    else if roll2 <= 88.0 { amount = 0.7}
                    else if roll2 <= 91.0 { amount = 0.65}
                    else if roll2 <= 94.0 { amount = 0.60}
                    else if roll2 <= 97.0 { amount = 0.85}
                    else { amount = 0.5}


                    WorkModule::set_float(fighter.module_accessor, amount, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_POWER_MULT);
                    WorkModule::set_int(fighter.module_accessor, 720, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_TIMER);

                    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF) != 0 {
                        let projeffect: u32 = WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF) as u32;
                        EffectModule::kill(fighter.module_accessor, projeffect, false, false);
                        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF);
                    }

                    if roll2 < 70.0 {
                        EffectModule::req_follow(fighter.module_accessor, Hash40::new("brave_damageup"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    }
                }
                else if roll1 == 1 { //heal
                    let mut amount = 0.0;
                    if roll2 <= 30.0 { amount = -25.0}
                    else if roll2 <= 40.0 { amount = -20.0}
                    else if roll2 <= 45.0 { amount = -10.0}
                    else if roll2 <= 50.0 { amount = -5.0}
                    else if roll2 <= 55.0 { amount = -25.0}
                    else if roll2 <= 60.0 { amount = -30.0}
                    else if roll2 <= 65.0 { amount = -35.0}
                    else if roll2 <= 68.0 { amount = -2.0}
                    else if roll2 <= 69.0 { amount = -1.0}
                    else if roll2 <= 70.0 { amount = -50.0}

                    else if roll2 <= 82.0 { amount = 20.0}
                    else if roll2 <= 85.0 { amount = 10.0}
                    else if roll2 <= 88.0 { amount = 5.0}
                    else if roll2 <= 91.0 { amount = 25.0}
                    else if roll2 <= 94.0 { amount = 30.0}
                    else if roll2 <= 97.0 { amount = 35.0}
                    else if roll2 <= 98.0 { amount = 2.0}
                    else if roll2 <= 99.0 { amount = 1.0}
                    else { amount = -50.0}
                    DamageModule::heal(fighter.module_accessor, amount, 0);
                    if amount < 0.0 {
                        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_recovery"), Hash40::new("top"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    } else {
                        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_dead"), Hash40::new("hip"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                    }
                }

                //fighter.sub_change_motion_by_situation(Hash40::new("special_lw_start").into(), Hash40::new("special_air_lw_start").into(), false.into());

                else if roll1 == 2 { //wtf
                    /* 
                    0  sleep -
                    1  invis =
                    2  flower -
                    3  poison -
                    4  slow -
                    5  starman +
                    6  giant +
                    7  tiny -
                    8 (18) kamikaze -
                    9 full mp +
                    10 (20) acceleratle +
                    11 (22) bounce +
                    12 x final smash +
                    13 x bury -
                    14 x ice -
                    15 x shield break -
                    16 x paralyze -
                    */     



                    let mut hocuspocus = -1;

                    if roll2 < 30.0 {
                        let mut rand_spell = sv_math::rand(hash40("brave"), 9) as i32; //0-8

                        if roll2 == 1.0 {
                            //rand_spell = 18; //kamikaze
                            fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
                        }
                        else if rand_spell == 1 {
                            fighter.change_status(FIGHTER_STATUS_KIND_BURY.into(), false.into()); //bury
                        }
                        else if rand_spell == 5 {
                            fighter.change_status(FIGHTER_STATUS_KIND_ICE.into(), false.into()); //ice
                        }
                        else if rand_spell == 6 {
                            fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into()); //shield break
                        }
                        else if rand_spell == 8 {
                            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into()); //paralyze (crumple)
                        }
                        else {
                            hocuspocus = rand_spell;
                        }
                    }
                    else {
                        let mut rand_spell = sv_math::rand(hash40("brave"), 7) as i32; //0-6

                        if rand_spell == 0 {
                            hocuspocus = 6;
                        }
                        else if rand_spell == 6 {
                            hocuspocus = 9;
                            WorkModule::set_int(fighter.module_accessor, 3600, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
                        }
                        else if rand_spell == 2 {
                            hocuspocus = 20;
                        }
                        else if rand_spell == 3 {
                            hocuspocus = 22;
                        }
                        else if rand_spell == 4 {
                            let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                            LookupSymbol(
	                        	&mut FIGHTER_MANAGER_ADDR,
	                        	"_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
	                        		.as_bytes()
	                        		.as_ptr(),
	                        );
	                        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
                            smash::app::lua_bind::FighterManager::set_final(fighter_manager,smash::app::FighterEntryID(entry.try_into().unwrap()),smash::app::FighterAvailableFinal { _address: *(smash::lib::lua_const::FighterAvailableFinal::DEFAULT) as u8 },0);
                        }
                        else {
                            hocuspocus = rand_spell;
                        }
                    }

                    println!("hocuspocus {}", hocuspocus);
                    if hocuspocus > 0 {
                        WorkModule::set_int(fighter.module_accessor, 14, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
                        WorkModule::set_int(fighter.module_accessor, hocuspocus, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_VARIOUS_KIND);

                        let fobj = fighter.global_table[0x4].get_ptr() as *mut Fighter;
                        let lwcommand: smash::app::FighterBraveSpecialLwCommand = smash::app::FighterBraveSpecialLwCommand { _address: 14 };
                        //FighterSpecializer_Brave::get_special_lw_command_from_index(fobj, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX)
                        FighterSpecializer_Brave::special_lw_decide_command(fobj, lwcommand, 0);

                        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT.into(), true.into());
                    }

                        

                    /* 
                    //let mut rand_spell = sv_math::rand(hash40("brave"), 12) as i32; //0-11
                    let mut rand_spell = sv_math::rand(hash40("brave"), 17) as i32; //0-16
                    if rand_spell < 12 { //hocus pocus stuff
                        if rand_spell == 8 {
                            rand_spell = 18; //kamikaze
                        } else if rand_spell == 10 {
                            rand_spell = 20; //acceleratle
                        } else if rand_spell == 11 {
                            rand_spell = 22; //bounce
                        } /* else if rand_spell == 12 {
                            rand_spell = 17;
                        } */

                        WorkModule::set_int(fighter.module_accessor, 14, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
                        WorkModule::set_int(fighter.module_accessor, rand_spell, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_VARIOUS_KIND);

                        let fobj = fighter.global_table[0x4].get_ptr() as *mut Fighter;
                        let lwcommand: smash::app::FighterBraveSpecialLwCommand = smash::app::FighterBraveSpecialLwCommand { _address: 14 };
                        //FighterSpecializer_Brave::get_special_lw_command_from_index(fobj, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX)
                        FighterSpecializer_Brave::special_lw_decide_command(fobj, lwcommand, 0);

                        //fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START.into(), true.into());
                        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT.into(), true.into());
                    }
                    if rand_spell == 12 { //final smash
                        let entry = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        LookupSymbol(
	                    	&mut FIGHTER_MANAGER_ADDR,
	                    	"_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
	                    		.as_bytes()
	                    		.as_ptr(),
	                    );
	                    let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
                        smash::app::lua_bind::FighterManager::set_final(fighter_manager,smash::app::FighterEntryID(entry.try_into().unwrap()),smash::app::FighterAvailableFinal { _address: *(smash::lib::lua_const::FighterAvailableFinal::DEFAULT) as u8 },0);
                    }
                    if rand_spell == 9 { //full mp?
                        WorkModule::set_int(fighter.module_accessor, 3600, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
                        //use ATTACK_ABS for these? (need to do all this hitbox/effect stuff in acmd)
                    }
                    if rand_spell == 13 { //bury
                        //WorkModule::set_int(fighter.module_accessor, 13, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POTION_TYPE);
                        //use ATTACK_ABS for these? (need to do all this hitbox/effect stuff in acmd)
                        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_hi_3"), 2.0, 1.0, false, 0.0, false, false);
                        fighter.change_status(FIGHTER_STATUS_KIND_BURY.into(), false.into());
                        //StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_BURY, true);
                    }
                    if rand_spell == 14 { //ice
                        //WorkModule::set_int(fighter.module_accessor, 14, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POTION_TYPE);
                        fighter.change_status(FIGHTER_STATUS_KIND_ICE.into(), false.into());
                    }
                    if rand_spell == 15 { //shield break
                        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
                    }
                    if rand_spell == 16 { //paralyze?
                        //WorkModule::set_int(fighter.module_accessor, 16, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POTION_TYPE);
                        fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
                    }
                } */
                }



                WorkModule::off_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_POTION_DRINK);
            }
        }
        else {
            if situation_kind != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
    }
    return 0.into();
}


unsafe extern "C" fn empty_acmd(agent: &mut L2CAgentBase) {
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
        .game_acmd("game_specialhistart", megumin_upb, Default)
        .game_acmd("game_specialairhistart", megumin_upb, Default)
        .sound_acmd("sound_specialhistart", megumin_upb_sound, Default)
        .sound_acmd("sound_specialairhistart", megumin_upb_sound, Default)
        /* .effect_acmd("effect_specialhistart", megumin_upb_eff, Default)
        .game_acmd("game_specialairhistart", megumin_upb_air, Default)
        .effect_acmd("effect_specialairhistart", megumin_upb_eff, Default)
        .sound_acmd("sound_specialairhistart", megumin_upb_sound, Default) */
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, megumin_upb_pre)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, megumin_upb_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, megumin_upb_main)
        
        .game_acmd("game_specialnfail", megumin_neutralb0, Default)
        .game_acmd("game_specialn1", megumin_neutralb1, Default)
        .game_acmd("game_specialn2", megumin_neutralb2, Default)
        .game_acmd("game_specialn3", megumin_neutralb3, Default)
        .game_acmd("game_specialairnfail", megumin_neutralb0, Default)
        .game_acmd("game_specialairn1", megumin_neutralb1, Default)
        .game_acmd("game_specialairn2", megumin_neutralb2, Default)
        .game_acmd("game_specialairn3", megumin_neutralb3, Default)
        .effect_acmd("effect_specialnfail", megumin_neutralb0_eff, Default)
        //.effect_acmd("effect_specialn1", megumin_neutralb1_eff, Default)
        //.effect_acmd("effect_specialn2", megumin_neutralb2_eff, Default)
        //.effect_acmd("effect_specialn3", megumin_neutralb3_eff, Default)
        .effect_acmd("effect_specialairnfail", megumin_neutralb0_eff, Default)
        //.effect_acmd("effect_specialairn1", megumin_neutralb1_eff, Default)
        //.effect_acmd("effect_specialairn2", megumin_neutralb2_eff, Default)
        //.effect_acmd("effect_specialairn3", megumin_neutralb3_eff, Default)
        .sound_acmd("sound_specialnfail", sound_neutralb0, Default)
        .sound_acmd("sound_specialn1", sound_neutralb1, Default)
        .sound_acmd("sound_specialn2", sound_neutralb2, Default)
        .sound_acmd("sound_specialn3", sound_neutralb3, Default)
        .sound_acmd("sound_specialairnfail", sound_neutralb0, Default)
        .sound_acmd("sound_specialairn1", sound_neutralb1, Default)
        .sound_acmd("sound_specialairn2", sound_neutralb2, Default)
        .sound_acmd("sound_specialairn3", sound_neutralb3, Default)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, megumin_neutralb_pre)
        //.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, megumin_neutralb_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, megumin_neutralb_main)

        .game_acmd("game_specials1", megumin_sideb1, Default)
        .game_acmd("game_specialairs1", megumin_sideb1, Default)
        .effect_acmd("effect_specials1", empty_acmd, Default)
        .effect_acmd("effect_specialairs1", empty_acmd, Default)
        .sound_acmd("sound_specials1", sound_sideb1, Default)
        .sound_acmd("sound_specialairs1", sound_sideb1, Default)
        //.effect_acmd("effect_specialnfail", megumin_neutralb0_eff, Default)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, megumin_sideb_pre)
        //.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, megumin_neutralb_init)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, megumin_sideb_main)
        
        .game_acmd("game_speciallwstart", megumin_downb1, Default)
        .game_acmd("game_specialairlwstart", megumin_downb1_air, Default)
        //.effect_acmd("effect_speciallwstart", megumin_downb1_eff, Default)
        //.effect_acmd("effect_specialairlwstart", megumin_downb1_eff, Default)
        .game_acmd("game_speciallwcancel", megumin_downb2, Default)
        .game_acmd("game_specialairlwcancel", megumin_downb2, Default)
        .sound_acmd("sound_speciallwcancel", megumin_downb2_sound, Default)
        .sound_acmd("sound_specialairlwcancel", megumin_downb2_sound, Default)
        //.sound_acmd("sound_speciallw9", sound_kamikaze, Default)
        //.sound_acmd("sound_specialairlw9", sound_kamikaze, Default)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, megumin_downb_main)
        .install();
    Agent::new("brave_fireball")
    .set_costume(costume.to_vec())
        .game_acmd("game_specialn1", side_b_fireball_move, Default)
        .game_acmd("game_bursts", side_b_fireball_burst, Default)
        .status(Init, *WEAPON_BRAVE_FIREBALL_STATUS_KIND_FLY_S, megumin_sideb_bullet_init)
        //.status(Main, *WEAPON_BRAVE_FIREBALL_STATUS_KIND_BURST_S, megumin_sideb_burst_main)
        .install();
    Agent::new("brave_explosion")
    .set_costume(costume.to_vec())
        .game_acmd("game_move2", game_move2, Default)
        .effect_acmd("effect_move2", effect_move2, Default)
        .game_acmd("game_explode2", neutralb_proj_explode1, Default)
        .effect_acmd("effect_explode2", eff_explode1, Default)
        .game_acmd("game_move1", neutralb_proj_explode2, Default)
        .effect_acmd("effect_move1", eff_explode2, Default)
        .game_acmd("game_explode1", neutralb_proj_explode3, Default)
        .effect_acmd("effect_explode1", eff_explode3, Default)
        .sound_acmd("sound_explode2", empty_acmd, Default)
        .sound_acmd("sound_move1", empty_acmd, Default)
        .sound_acmd("sound_explode1", empty_acmd, Default)
        .status(Pre, *WEAPON_BRAVE_EXPLOSION_STATUS_KIND_MOVE2, neutralb_proj_pre)
        .status(Main, *WEAPON_BRAVE_EXPLOSION_STATUS_KIND_MOVE2, neutralb_proj_main)
        .status(Main, *WEAPON_BRAVE_EXPLOSION_STATUS_KIND_EXPLODE2, neutralb_proj_explode_main)
        .install();
    Agent::new("brave_flash")
    .set_costume(costume.to_vec())
        .game_acmd("game_flash1", empty_acmd, Default)
        .effect_acmd("effect_flash1", empty_acmd, Default)
        .game_acmd("game_flashhit1", game_flashhit1, Default)
        //.effect_acmd("effect_flashhit1", empty_acmd, Default)
        .status(Main, *WEAPON_BRAVE_FLASH_STATUS_KIND_START1, downb_proj_main)
        .status(Main, *WEAPON_BRAVE_FLASH_STATUS_KIND_HIT1, downb_proj_explode_main)
        .install();
}