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






unsafe extern "C" fn game_finalstart(agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn effect_finalstart(agent: &mut L2CAgentBase) {
    
}



unsafe extern "C" fn megumin_finalstart_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));
    
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_start"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_finalstart_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_finalstart_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_UP.into(), false.into());
    }
    return 0.into();
}

unsafe extern "C" fn megumin_finalstart_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}



unsafe extern "C" fn megumin_zoom_land_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if PostureModule::pos_x(fighter.module_accessor) < 0.0 {
        PostureModule::set_lr(fighter.module_accessor, 1.0);
    }
    else if PostureModule::pos_x(fighter.module_accessor) > 0.0 {
        PostureModule::set_lr(fighter.module_accessor, -1.0);
    }
    
    fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_FINAL_END.into(), false.into());
    return 0.into();
}




unsafe extern "C" fn megumin_zoom_down_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    /* let ret = original_status(Main, fighter, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_DOWN) (fighter);
    WorkModule::set_int(fighter.module_accessor, 999, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_INT_ARRIVE_TIME);
    ret */

    GroundModule::set_collidable(fighter.module_accessor, true);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_STOP_CEIL);

    let zoompos = PostureModule::pos(fighter.module_accessor);
    let newx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_FLOAT_TARGET_POS_X);
    let mut newpos = Vector3f{x:newx, y:(*zoompos).y, z:(*zoompos).z};
    PostureModule::init_pos(fighter.module_accessor, &newpos, true, true);

    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_OFF), 0);
    lua_args!(fighter, Hash40::new_raw(0x1e61567377));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.clear_lua_stack();

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw16_lw"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    let down_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("flying_down_speed_y"));
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    //KineticModule::enable_energy(module_accessor,*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, down_y * -1.0);
    sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_stable_speed(fighter.lua_state_agent);
    sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
    sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    sv_kinetic_energy::set_accel(fighter.lua_state_agent);

    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_INT_COUNTER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_FLAG_CORRECT_CHANGED);

    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_zoom_down_main_loop as *const () as _))
}


unsafe extern "C" fn megumin_zoom_down_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_FLAG_ARRIVED) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            //if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                //change status to landing
            //}
        //}
    //} else {
        fighter.change_status(FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_END.into(), false.into());
    }
    return 0.into();
}


unsafe extern "C" fn megumin_zoom_down_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_FLAG_CORRECT_CHANGED) {
        GroundModule::correct(fighter.module_accessor, smash::app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_LW_FLYING_FLAG_CORRECT_CHANGED);
    }
    return 0.into();
}


unsafe extern "C" fn megumin_zoom_up_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32,smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_VISIBILITY) as u32);
    return 0.into()
}

unsafe extern "C" fn megumin_zoom_down_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_NONE as u32,smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_VISIBILITY) as u32);
    return 0.into()
}

unsafe extern "C" fn megumin_zoom_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_AIR as u32,smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, (*FS_SUCCEEDS_KEEP_HIT | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_VISIBILITY) as u32);
    return 0.into()
}










unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
        macros::ATTACK(agent, 4, 2, Hash40::new("ezkudu"), 5.0, 155, 66, 0, 66, 5.0, 0.0, 0.0, 0.0, Some(0.0), Some(150.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0.0, 0.0, 30, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    for i in 0..936 { //WAS 946
        if macros::is_excute(agent) {
            let dmgtaken = WorkModule::get_float(agent.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_TAKEN);
            println!("dmgtaken acmd {}", dmgtaken);
            damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, (150.0 - dmgtaken) as i32);
        }
        wait(agent.lua_state_agent, 1.0);
        if i % 30 == 0 {
            if macros::is_excute(agent) {
                //macros::ATTACK(agent, 3, 1, Hash40::new("ezkudu"), 0.0, 366, (i / 13) + 30, 22, 0, 154.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 5, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
                macros::ATTACK(agent, 3, 1, Hash40::new("ezkudu"), 0.0, 366, (i / 65) + 30, 22, 0, 154.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 1, false, false, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
        }
    }
    frame(agent.lua_state_agent, 940.0); //WAS 950
    //macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        damage!(agent, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        macros::ATTACK(agent, 0, 0, Hash40::new("ezkudu"), 900.4, 54, 80, 0, 45, 153.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_FINAL_FLAG_END_FINAL);
        //notify_event_msc_cmd!(agent, Hash40::new_raw(0x1e0aba2d68));
    }
}

unsafe extern "C" fn effect_final(agent: &mut L2CAgentBase) {
    let mut explosioneffect: u32 = 0;
    if macros::is_excute(agent) {
        macros::FILL_SCREEN_MODEL_COLOR(agent, 1, 0, 0.14, 0.16, 0.18, 0, 0, 0, 1, 1.2, *smash::lib::lua_const::EffectScreenLayer::GROUND, *EFFECT_SCREEN_PRIO_FINAL);
    }
    
    frame(agent.lua_state_agent, 935.0);
    if macros::is_excute(agent) {
        let mut pos = Vector3f{x:0.0,y:0.0,z:0.0};
        let joint_offset = ModelModule::joint_global_position(agent.module_accessor, Hash40::new("ezkudu"), &mut pos,false); 
        explosioneffect = EffectModule::req(agent.module_accessor, Hash40::new("brave_fullburst"), &pos, &Vector3f{x:0.0,y:90.0,z:0.0}, 3.6, 0, -1, false, 0) as u32;
        EffectModule::set_rgb(agent.module_accessor, explosioneffect, 1.0, 1.0, 0.5);
        EffectModule::set_rate(agent.module_accessor, explosioneffect, 1.2);
    }
    frame(agent.lua_state_agent, 955.0);
    if macros::is_excute(agent) {
        macros::CANCEL_FILL_SCREEN(agent, 1, 30);
    }
    
}



unsafe extern "C" fn sound_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_brave_final01"));
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_brave_final01"));
    }
}

unsafe extern "C" fn megumin_final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, smash::app::SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_DISABLE, false, true, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 
        (*FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | *FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | *FIGHTER_STATUS_ATTR_FINAL) as u32, *FIGHTER_POWER_UP_ATTACK_BIT_FINAL as u32, 0);
    return 0.into();
}

unsafe extern "C" fn megumin_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dmg = DamageModule::damage(fighter.module_accessor, 0);
    WorkModule::set_float(fighter.module_accessor, 0.0, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_TAKEN);
    WorkModule::set_float(fighter.module_accessor, dmg, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_START);

    /* lua_args!(fighter, Hash40::new_raw(0x201bc9217c));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.clear_lua_stack(); */

    //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x201bc9217c));

    MotionModule::change_motion(fighter.module_accessor, Hash40::new("final_end"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(megumin_final_main_loop as *const () as _))
}

unsafe extern "C" fn megumin_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();

    println!("frame status {}", fighter.global_table[STATUS_FRAME].get_f32());
    println!("frame motion {}", fighter.global_table[STATUS_FRAME].get_f32());

    if /* StopModule::is_stop(fighter.module_accessor) || */ StopModule::is_hit(fighter.module_accessor) && MotionModule::frame(fighter.module_accessor) < 899.0 {
        StopModule::cancel_hit_stop(fighter.module_accessor);
    }

    let dmg = DamageModule::damage(fighter.module_accessor, 0);
    let dmgstart = WorkModule::get_float(fighter.module_accessor, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_START);
    if dmg > dmgstart {
        WorkModule::set_float(fighter.module_accessor, dmg - dmgstart, FIGHTER_MEGUMIN_INSTANCE_WORK_ID_FLOAT_FINAL_DAMAGE_TAKEN);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            //fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            fighter.change_status(FIGHTER_STATUS_KIND_SAVING_DAMAGE.into(), false.into());
        }
        //notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));
    }
    return 0.into();
}

unsafe extern "C" fn megumin_final_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e0aba2d68));

    return 0.into();
}

//0x1e0aba2d68

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
        /* .game_acmd("game_finalstart", game_final, Default)
        .effect_acmd("effect_finalstart", effect_finalstart, Default)
        .sound_acmd("sound_finalstart", sound_finalstart, Default)
        .status(Pre, *FIGHTER_STATUS_KIND_FINAL, megumin_final_pre)
        .status(Main, *FIGHTER_STATUS_KIND_FINAL, megumin_final_main) */
        .status(Pre, *FIGHTER_STATUS_KIND_FINAL, megumin_final_pre)

        .status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_UP, megumin_zoom_up_pre)
        .status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_DOWN, megumin_zoom_down_pre)
        .status(Pre, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_END, megumin_zoom_end_pre)
        .status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_DOWN, megumin_zoom_down_main)
        //.status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_UP, megumin_zoom_up_main)
        .status(Exec, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_DOWN, megumin_zoom_down_exec)
        .status(Main, *FIGHTER_BRAVE_STATUS_KIND_SPECIAL_LW_FLYING_END, megumin_zoom_land_main)

        .game_acmd("game_finalstart", game_finalstart, Default)
        .effect_acmd("effect_finalstart", effect_finalstart, Default)
        //.effect_acmd("effect_finalstart", effect_finalstart, Default)
        //.sound_acmd("sound_finalstart", sound_finalstart, Default)
        .status(Main, *FIGHTER_STATUS_KIND_FINAL, megumin_finalstart_main)
        .status(End, *FIGHTER_STATUS_KIND_FINAL, megumin_finalstart_end)
        .game_acmd("game_finalend", game_final, Default)
        .effect_acmd("effect_finalend", effect_final, Default)
        .sound_acmd("sound_finalend", sound_final, Default)
        .status(Pre, *FIGHTER_BRAVE_STATUS_KIND_FINAL_END, megumin_final_pre)
        .status(Main, *FIGHTER_BRAVE_STATUS_KIND_FINAL_END, megumin_final_main)
        .status(Exit, *FIGHTER_BRAVE_STATUS_KIND_FINAL_END, megumin_final_end)
        .status(End, *FIGHTER_BRAVE_STATUS_KIND_FINAL_END, megumin_final_end)
        
        .install();
}

