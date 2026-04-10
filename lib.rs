#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_imports,
	unused_macros,
	unused_variables,
	unused_assignments,
	unused_unsafe,
	non_upper_case_globals,
	non_snake_case,
    clippy::borrow_interior_mutable_const,
	unused_parens,
	unused_mut,
	dead_code,
    static_mut_refs
)]

use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};


//mod EDIT;
mod normals;
mod specials;
mod finalsmash;
mod opff;





pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const FIGHTER_NAME: &str = "brave";
    const MARKER_FILE: &str = "megumin.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/{}/model/body/c{:02}/{}",
            FIGHTER_NAME, x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }
	
	param_config::update_int_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(), (hash40("attack_combo_max"), 0, 1));
	param_config::update_int_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(), (hash40("s3_combo_max"), 0, 1));
	param_config::update_int_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(), (hash40("combo_attack_32_end"), 0, 0));
	
	//param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(), (hash40("param_special_lw"), hash40("flying_fall_enable_shape_offset_y"), -333.0));

	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("walk_accel_mul"), 0, 0.189));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("walk_speed_max"), 0, 0.784));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("dash_speed"), 0, 2.15));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("run_accel_mul"), 0, 0.09));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("run_accel_add"), 0, 0.044));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("run_speed_max"), 0, 2.1));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("jump_initial_y"), 0, 15.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("jump_y"), 0, 30.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("mini_jump_y"), 0, 14.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("jump_aerial_y"), 0, 25.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_accel_x_mul"), 0, 0.06));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_speed_x_stable"), 0, 1.17));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_brake_x"), 0, 0.00375));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_accel_y"), 0, 0.105));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_speed_y_stable"), 0, 1.87));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("damage_fly_top_air_accel_y"), 0, 0.07224));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_brake_y"), 0, 0.004));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("dive_speed_y"), 0, 2.618));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("weight"), 0, 80.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("air_ground_speed_brake"), 0, 0.01125));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("landing_attack_air_frame_n"), 0, 7.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("landing_attack_air_frame_f"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("landing_attack_air_frame_b"), 0, 10.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("landing_attack_air_frame_hi"), 0, 8.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("landing_attack_air_frame_lw"), 0, 1.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("shield_radius"), 0, 9.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("shield_brake_y"), 0, 80.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("jostle_front"), 0, 1.3));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("jostle_back"), 0, 1.8));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("jostle_weight"), 0, 5.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("cliff_jump_y"), 0, 30.0));
	
	//param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("size"), 0.1));
	/* param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("offset1_x"), 999.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("offset2_x"), 999.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("offset1_y"), 999.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("offset2_y"), 999.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("offset1_z"), 999.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("wait_shield_data"), hash40("offset2_z"), 999.0));

	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("param_shield"), hash40("wait_shield_scale_x"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("param_shield"), hash40("wait_shield_scale_y"), 0.0));
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("param_shield"), hash40("wait_shield_scale_z"), 0.0)); */
	param_config::update_float_2(*FIGHTER_KIND_BRAVE, marked_slots.clone(),(hash40("param_shield"), hash40("shield_guard_degree"), 0.0));

	param_config::disable_villager_pocket(*FIGHTER_KIND_BRAVE, marked_slots.clone(), 0);
	/* param_config::disable_villager_pocket(*FIGHTER_KIND_BRAVE, vec![-1].clone(), *WEAPON_KIND_BRAVE_EXPLOSION);
	param_config::disable_villager_pocket(*FIGHTER_KIND_BRAVE, vec![-1].clone(), *WEAPON_KIND_BRAVE_FIREBALL);
	param_config::disable_villager_pocket(*FIGHTER_KIND_BRAVE, vec![-1].clone(), *WEAPON_KIND_BRAVE_FLASH); */
	param_config::disable_kirby_copy(*FIGHTER_KIND_BRAVE, marked_slots.clone());


	


    if lowest_color == -1 {
        return;
    }

    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    the_csk_collection_api::add_narration_characall_entry("vc_narration_characall_megumin");

	the_csk_collection_api::add_chara_db_entry_info(
        the_csk_collection_api::CharacterDatabaseEntry {
            ui_chara_id: smash::hash40("ui_chara_megumin"),
            clone_from_ui_chara_id: None,
            name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("megumin")), 
            fighter_kind: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_brave")),
            fighter_kind_corps: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_kind_brave")),
            ui_series_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_series_konosuba")),
            fighter_type: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("fighter_type_normal")),
            alt_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0x0), 
            exhibit_year: the_csk_collection_api::ShortType::Overwrite(2002),
            exhibit_day_order: the_csk_collection_api::IntType::Overwrite(81109), 
            ext_skill_page_num: the_csk_collection_api::SignedByteType::Overwrite(1), 
            is_img_ext_skill_page0: the_csk_collection_api::BoolType::Overwrite(false), 
			is_img_ext_skill_page1: the_csk_collection_api::BoolType::Overwrite(false), 
			is_img_ext_skill_page2: the_csk_collection_api::BoolType::Overwrite(false),
            skill_list_order: the_csk_collection_api::SignedByteType::Optional(Some(81)), 
            disp_order: the_csk_collection_api::SignedByteType::Optional(Some(73)), 
            save_no: the_csk_collection_api::SignedByteType::Overwrite(78), 
			chara_count: the_csk_collection_api::SignedByteType::Overwrite(1), 
            can_select: the_csk_collection_api::BoolType::Overwrite(true), 
			is_usable_soundtest: the_csk_collection_api::BoolType::Overwrite(true), 
			is_called_pokemon: the_csk_collection_api::BoolType::Overwrite(false), 
			is_mii: the_csk_collection_api::BoolType::Overwrite(false), 
			is_boss: the_csk_collection_api::BoolType::Overwrite(false), 
			is_hidden_boss: the_csk_collection_api::BoolType::Overwrite(false), 
			is_dlc: the_csk_collection_api::BoolType::Overwrite(false), 
			is_patch: the_csk_collection_api::BoolType::Overwrite(false), 
            is_plural_message: the_csk_collection_api::BoolType::Overwrite(false), 
			is_plural_narration: the_csk_collection_api::BoolType::Overwrite(false), 
			is_article: the_csk_collection_api::BoolType::Overwrite(false), 
            extra_flags: the_csk_collection_api::IntType::Overwrite(0), 
            //unk_0x112b7bb52a: the_csk_collection_api::BoolType::Overwrite(false),
            has_multiple_face: the_csk_collection_api::BoolType::Overwrite(false),
            result_pf0: the_csk_collection_api::BoolType::Overwrite(true), 
			result_pf1: the_csk_collection_api::BoolType::Overwrite(true), 
			result_pf2: the_csk_collection_api::BoolType::Overwrite(true), 
			color_num: the_csk_collection_api::UnsignedByteType::Overwrite(color_num as u8), 
            extra_index_maps: the_csk_collection_api::UnsignedByteMap::Overwrite(std::collections::HashMap::from([
				(0x915C075DE /* c00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9B3B77E6A /* c01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9825F64F7 /* c02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x924286F43 /* c03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9E18F51CD /* c04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x947F85A79 /* c05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9761040E4 /* c06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9D0674B50 /* c07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9E48F9289 /* n00_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x942F8993D /* n01_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9731083A0 /* n02_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9D5678814 /* n03_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x910C0B69A /* n04_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9B6B7BD2E /* n05_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9875FA7B3 /* n06_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x92128AC07 /* n07_index */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9F873561A /* c00_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x95E045DAE /* c01_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x96FEC4733 /* c02_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9C99B4C87 /* c03_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x90C3C7209 /* c04_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x9AA4B79BD /* c05_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x99BA36320 /* c06_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x93DD46894 /* c07_group */, the_csk_collection_api::UnsignedByteType::Overwrite(0)), 
				(0x11895f00fc, the_csk_collection_api::UnsignedByteType::Overwrite(lowest_color as _)),
			])), 
            extra_hash_maps: the_csk_collection_api::Hash40Map::Overwrite(std::collections::HashMap::from([
				(0x1337FC912E /* characall_label_c00 */, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("vc_narration_characall_megumin"))), 
				(0x1340FBA1B8 /* characall_label_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13D9F2F002 /* characall_label_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13AEF5C094 /* characall_label_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1330915537 /* characall_label_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13479665A1 /* characall_label_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13DE9F341B /* characall_label_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x13A998048D /* characall_label_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B8B13E500 /* characall_label_article_c00 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1BFC14D596 /* characall_label_article_c01 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B651D842C /* characall_label_article_c02 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B121AB4BA /* characall_label_article_c03 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B8C7E2119 /* characall_label_article_c04 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1BFB79118F /* characall_label_article_c05 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B62704035 /* characall_label_article_c06 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x1B157770A3 /* characall_label_article_c07 */, the_csk_collection_api::Hash40Type::Overwrite(0x0)), 
				(0x160ab9eb98, the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_brave"))),
			])), 
            shop_item_tag: the_csk_collection_api::Hash40Type::Overwrite(0x0), 
        },
    );

	the_csk_collection_api::add_chara_layout_db_entry_info(the_csk_collection_api::CharacterLayoutDatabaseEntry {
		ui_layout_id: smash::hash40("ui_chara_megumin_00"), 
		ui_chara_id: the_csk_collection_api::Hash40Type::Overwrite(smash::hash40("ui_chara_megumin")),
		chara_color: the_csk_collection_api::UnsignedByteType::Optional(Some(0)), 
		eye_0_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(1)), 
		eye_1_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(1)), 
		eye_2_flash_count: the_csk_collection_api::UnsignedByteType::Optional(Some(1)), 
		eye_0_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(-20.0)), 
		eye_0_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(184.0)), 
		eye_0_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_0_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(44.0)), 
		eye_1_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(194.0)), 
		eye_1_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_1_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash0_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash0_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash1_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash1_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash2_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash2_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash3_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash3_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash4_pos_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_2_flash4_pos_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		eye_flash_info_pos_x: the_csk_collection_api::FloatType::Optional(Some(-3.0)), 
		eye_flash_info_pos_y: the_csk_collection_api::FloatType::Optional(Some(2.0)), 
		chara_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
		chara_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-65.0)), 
		chara_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
		chara_1_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(20.0)), 
		chara_1_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-60.0)), 
		chara_1_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.5)), 
		chara_1_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_1_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(8.0)), 
		chara_1_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-34.0)), 
		chara_1_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.42)), 
		chara_1_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-40.0)), 
		chara_1_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.43)), 
		chara_1_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_1_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		//chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(130.0)), 
		//chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-300.0)), 
		chara_3_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(50.0)), 
		chara_3_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(-100.0)), 
		//chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(0.75)), 
		chara_3_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(190.0)), 
		//chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-300.0)), 
		chara_3_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(80.0)), 
		chara_3_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(-100.0)), 
		//chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(0.75)), 
		chara_3_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_3_2_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), //working
		chara_3_2_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_2_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
		//chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(100.0)), 
		//chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-200.0)), 
		chara_3_3_offset_x: the_csk_collection_api::FloatType::Optional(Some(40.0)), 
		chara_3_3_offset_y: the_csk_collection_api::FloatType::Optional(Some(-70.0)), 
		//chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(0.68)), 
		chara_3_3_scale: the_csk_collection_api::FloatType::Optional(Some(1.1)), 
		//chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(32.0)), 
		//chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-32.0)), 
		chara_3_4_offset_x: the_csk_collection_api::FloatType::Optional(Some(12.0)), 
		chara_3_4_offset_y: the_csk_collection_api::FloatType::Optional(Some(-12.0)), 
		//chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		chara_3_4_scale: the_csk_collection_api::FloatType::Optional(Some(1.9)), 
		//chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(140.0)), 
		//chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-340.0)), 
		chara_3_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(50.0)), 
		chara_3_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(-120.0)), 
		//chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(0.7)), 
		chara_3_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_3_6_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_3_6_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		//chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_3_6_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		//chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(100.0)), 
		//chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-240.0)), 
		chara_3_7_offset_x: the_csk_collection_api::FloatType::Optional(Some(40.0)), 
		chara_3_7_offset_y: the_csk_collection_api::FloatType::Optional(Some(-80.0)), 
		//chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(0.7)), 
		chara_3_7_scale: the_csk_collection_api::FloatType::Optional(Some(1.2)), 
		chara_5_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_5_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_5_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_select_icon_list_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_select_icon_list_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_select_icon_list_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_7_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_7_1_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_1_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_7_1_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		chara_0_offset_x: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_0_offset_y: the_csk_collection_api::FloatType::Optional(Some(0.0)), 
		chara_0_scale: the_csk_collection_api::FloatType::Optional(Some(1.0)), 
		spirits_eye_visible: the_csk_collection_api::BoolType::Optional(Some(true)), 
		..std::default::Default::default()
	});

	the_csk_collection_api::add_series_db_entry_info(the_csk_collection_api::SeriesDatabaseEntry {
        ui_series_id: smash::hash40("ui_series_konosuba"),
		name_id: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("konosuba")), 
        disp_order: the_csk_collection_api::SignedByteType::Overwrite(0), 
        disp_order_sound: the_csk_collection_api::SignedByteType::Overwrite(0), 
        save_no: the_csk_collection_api::SignedByteType::Overwrite(0), 
        shown_as_series_in_directory: the_csk_collection_api::BoolType::Overwrite(false),
        is_dlc: the_csk_collection_api::BoolType::Overwrite(false),
        is_patch: the_csk_collection_api::BoolType::Overwrite(false),
        dlc_chara_id: the_csk_collection_api::Hash40Type::Overwrite(0),
        is_use_amiibo_bg: the_csk_collection_api::BoolType::Overwrite(false),
        ..std::default::Default::default()
    });
	/* the_csk_collection_api::add_series_db_entry_info(the_csk_collection_api::SeriesDatabaseEntry {
        ui_series_id: hash40("ui_series_tetrisseries"),
        name_id: StringType::Overwrite(CStrCSK::new("tetrisseries")),
        disp_order: SignedByteType::Overwrite(0),
        disp_order_sound: SignedByteType::Overwrite(0),
        save_no: SignedByteType::Overwrite(0),
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..Default::default()
    }); */


	the_csk_collection_api::add_bgm_db_entry_info(&the_csk_collection_api::BgmDatabaseRootEntry {
        ui_bgm_id: hash40("ui_bgm_z98_f_megumin"),
        clone_from_ui_bgm_id: Some(hash40("ui_bgm_z98_f_brave")),
        stream_set_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("set_z98_f_megumin")),
        ..std::default::Default::default()
    });

    the_csk_collection_api::add_stream_set_entry_info(&the_csk_collection_api::StreamSetEntry { 
        stream_set_id: hash40("set_z98_f_megumin"),
        info0: the_csk_collection_api::Hash40Type::Overwrite(hash40("info_z98_f_megumin")),
        ..std::default::Default::default()
    });

    the_csk_collection_api::add_assigned_info_entry_info(&the_csk_collection_api::AssignedInfoEntry { 
        info_id: hash40("info_z98_f_megumin"),
        stream_id: the_csk_collection_api::Hash40Type::Overwrite(hash40("stream_z98_f_megumin")),
        condition: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_none")),
        condition_process: the_csk_collection_api::Hash40Type::Overwrite(hash40("sound_condition_process_add")),
        change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        menu_change_fadeout_frame: the_csk_collection_api::IntType::Overwrite(60),
        ..std::default::Default::default()
    });

    the_csk_collection_api::add_stream_property_entry_info(&the_csk_collection_api::StreamPropertyEntry {
        stream_id: hash40("stream_z98_f_megumin"),
        data_name0: the_csk_collection_api::StringType::Overwrite(the_csk_collection_api::CStrCSK::new("z98_f_megumin")),
        ..std::default::Default::default()
    });

    the_csk_collection_api::add_new_bgm_property_entry(&smash_bgm_property::BgmPropertyEntry {
        stream_name: hash40::Hash40::new("z98_f_megumin"),
        loop_start_ms: 0,
        loop_start_sample: 0,
        loop_end_ms: 0,
        loop_end_sample: 0,
        duration_ms: 5800,
        duration_sample: 278400
    });

    the_csk_collection_api::set_fighter_jingle(hash40("ui_chara_megumin"), "z98_f_megumin");



    //EDIT::install();
	normals::install();
	specials::install();
	finalsmash::install();
	opff::install();
}


#[skyline::main(name = "smashline_test")]
pub fn main() {
	unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
	unsafe {
        /* //allows online play
        extern "C" {
            fn allow_ui_chara_hash_online(ui_chara_hash: u64);
        }
        allow_ui_chara_hash_online(smash::hash40("ui_chara_megumin")); //ui_chara_tails */
    }
}