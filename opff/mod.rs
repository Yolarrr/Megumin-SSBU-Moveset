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
pub const FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_ALREADY_DID_DOWN_B_INTO_UP_B :i32 = 0x200000F5;

pub const WEAPON_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_LEVEL :i32 = 0x20000030;
pub const WEAPON_MEGUMIN_INSTANCE_WORK_ID_FLAG_EXPLOSION_AERIAL :i32 = 0x10000050;



unsafe extern "C" fn megumin_frame(fighter: &mut L2CFighterCommon) {

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SHIELD_INSTANCE_WORK_ID_FLAG_WAIT_SHIELD) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SHIELD_INSTANCE_WORK_ID_FLAG_WAIT_SHIELD);
    }

    let mut chargetime = WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);

    if chargetime < 3602 && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_SPECIAL_N
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ENTRY 
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_DEAD 
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_STANDBY 
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_REBIRTH {
        WorkModule::inc_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }
    if chargetime > 3611 {
        WorkModule::set_int(fighter.module_accessor, 3610, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_EXPLOSION_CHARGE_TIME);
    }



    //let spell_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_DECIDE_COMMAND);
    //let various_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_VARIOUS_KIND);
    //println!("spell kind {}", spell_kind);
    //println!("various kind {}", various_kind);

    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_SELECT {
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_START.into(), true.into());
    }

    let fobj = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    FighterSpecializer_Brave::set_sp(fobj, (chargetime as f32 / 36.0).floor(), true);
    //println!("lw command {}", FighterSpecializer_Brave::get_special_lw_command_from_index(fobj, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX));

    //_FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX

    if WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_TIMER) > 0 /* && WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_TIMER) < 719 */ {
        let power_mult = WorkModule::get_float(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_POWER_MULT);
        AttackModule::set_power_mul(fighter.module_accessor, power_mult);
        WorkModule::dec_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_TIMER);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF) == 0 {
            //let mut projeffect = 0;
            if power_mult > 1.0 {
                let projeffect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_attack_up"), Hash40::new("hip"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                WorkModule::set_int(fighter.module_accessor, projeffect as i32, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF);
            }
            else if power_mult < 1.0 {
                let projeffect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_status_down"), Hash40::new("hip"), &Vector3f{x:0.0,y:0.0,z:0.0}, &Vector3f{x:0.0,y:0.0,z:0.0}, 1.0, true, 0, 0, 0, 0, 0, true, true) as u32;
                WorkModule::set_int(fighter.module_accessor, projeffect as i32, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF);
            }
        }
    }
    else {
        AttackModule::set_power_mul(fighter.module_accessor, 1.0);
        if WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF) != 0 {
            let projeffect: u32 = WorkModule::get_int(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_INT_POWER_EFF) as u32;
            EffectModule::kill(fighter.module_accessor, projeffect, false, false);
        }
    }



    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_BURY && MotionModule::motion_kind(fighter.module_accessor) == 0 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("damage_lw_3"), 3.0, 0.0, false, 0.0, false, false);
    }

    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_S3 {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
    }
    
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_UP {
        GroundModule::set_collidable(fighter.module_accessor, false);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);
    }
    

    
    
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_DOWN_B_DISABLE_AIR) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_SPECIAL_LW);
        } else {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_DOWN_B_DISABLE_AIR);
        }
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR && !StopModule::is_hit(fighter.module_accessor) {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_SPECIAL_HI);
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR) 
    //&& (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND || (StopModule::is_hit(fighter.module_accessor)/*  && fighter.global_table[PREV_STATUS_KIND].get_i32() != FIGHTER_STATUS_KIND_SPECIAL_LW */)) {
    && (fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND || StopModule::is_hit(fighter.module_accessor) 
    || fighter.global_table[STATUS_KIND].get_i32() == FIGHTER_STATUS_KIND_CLIFF_CATCH || fighter.global_table[STATUS_KIND].get_i32() == FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR);
    }


    if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLAG_UP_B_DISABLE_AIR) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
        }

        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
        && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
            fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), true.into());
        }
        
        fighter.sub_transition_group_check_air_escape();
        fighter.sub_transition_group_check_ground_special();
        fighter.sub_transition_group_check_air_special();
    }
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
        .on_line(Main, megumin_frame)
        .install();
}








