#![allow(dead_code)]

use serde::Deserialize;

pub static COMMON_ABILITIES: [&str; 16] = [
    "Avatar_DefaultAbility_VisionReplaceDieInvincible",
    "Avatar_DefaultAbility_AvartarInShaderChange",
    "Avatar_SprintBS_Invincible",
    "Avatar_Freeze_Duration_Reducer",
    "Avatar_Attack_ReviveEnergy",
    "Avatar_Component_Initializer",
    "Avatar_FallAnthem_Achievement_Listener",
    "GrapplingHookSkill_Ability",
    "Avatar_PlayerBoy_DiveStamina_Reduction",
    "Ability_Avatar_Dive_Team",
    "Ability_Avatar_Dive_SealEcho",
    "Absorb_SealEcho_Bullet_01",
    "Absorb_SealEcho_Bullet_02",
    "Ability_Avatar_Dive_CrabShield",
    "ActivityAbility_Absorb_Shoot",
    "SceneAbility_DiveVolume",
];

#[derive(Deserialize)]
pub struct AvatarAbilityConfig {
    pub abilities: Vec<AbilityConfig>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityConfig {
    #[serde(rename = "abilityID")]
    pub ability_id: String,
    pub ability_name: String,
    pub ability_override: String,
}
