#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct AvatarExcelConfig {
    pub use_type: String,
    pub body_type: String,
    pub icon_name: String,
    pub side_icon_name: String,
    pub quality_type: String,
    pub charge_efficiency: f32,
    pub initial_weapon: u32,
    pub weapon_type: String,
    pub image_name: String,
    pub cutscene_show: String,
    pub skill_depot_id: u32,
    pub stamina_recover_speed: f32,
    pub cand_skill_depot_ids: Vec<u32>,
    pub manekin_motion_config: u32,
    pub avatar_identity_type: String,
    pub avatar_promote_id: u32,
    pub avatar_promote_reward_level_list: Vec<u32>,
    pub avatar_promote_reward_id_list: Vec<u32>,
    #[serde(rename = "featureTagGroupID")]
    pub feature_tag_group_id: u32,
    pub hp_base: f32,
    pub attack_base: f32,
    pub defense_base: f32,
    pub critical: f32,
    pub critical_hurt: f32,
    pub prop_grow_curves: Vec<PropGrowCurve>,
    pub id: u32,
    #[serde(rename = "LODPatternName")]
    pub lodpattern_name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropGrowCurve {
    #[serde(rename = "type")]
    pub curve_type: String,
    pub grow_curve: String,
}
