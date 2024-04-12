use proto::*;

use super::{prop_types::*, GUIDManager};
use crate::{data::EXCEL_COLLECTION, util};

pub struct PlayerInfo {
    pub uid: u32,
    pub guid_manager: GUIDManager,
    pub avatars: Vec<AvatarData>,
}

#[derive(Clone)]
pub struct AvatarData {
    pub guid: u64,
    pub avatar_id: u32,
    pub skill_depot_id: u32,
    pub wearing_flycloak_id: u32,
    pub costume_id: u32,
    pub born_time: u32,
    pub weapon: WeaponData,
    pub properties: Vec<PropValue>,
    pub fight_properties: Vec<FightPropPair>,
}

#[derive(Clone)]
pub struct WeaponData {
    pub guid: u64,
    pub weapon_id: u32,
}

static DISABLED_AVATAR_LIST: [u32; 3] = [10000001, 10000005, 10000007];
impl PlayerInfo {
    pub fn new() -> Self {
        Self {
            uid: 1337,
            guid_manager: GUIDManager::new(),
            avatars: Vec::new(),
        }
    }

    pub fn next_guid(&self) -> u64 {
        self.guid_manager.next()
    }

    pub fn unlock_all_avatars(&mut self) {
        for config in EXCEL_COLLECTION
            .avatar_configs
            .iter()
            .filter(|c| !DISABLED_AVATAR_LIST.contains(&c.id))
        {
            let mut data = AvatarData {
                guid: self.next_guid(),
                avatar_id: config.id,
                skill_depot_id: config.skill_depot_id,
                wearing_flycloak_id: 140001,
                costume_id: 0,
                born_time: util::cur_timestamp() as u32,
                weapon: WeaponData {
                    guid: self.next_guid(),
                    weapon_id: config.initial_weapon,
                },
                properties: Vec::new(),
                fight_properties: Vec::new(),
            };

            data.set_property(PROP_LEVEL, 1);
            data.set_property(PROP_EXP, 0);
            data.set_property(PROP_BREAK_LEVEL, 0);

            let base_hp = config.hp_base;
            let base_attack = config.attack_base;
            let base_defense = config.defense_base;

            data.set_fight_property(FIGHT_PROP_BASE_HP, base_hp);
            data.set_fight_property(FIGHT_PROP_CUR_HP, base_hp);
            data.set_fight_property(FIGHT_PROP_MAX_HP, base_hp);

            data.set_fight_property(FIGHT_PROP_BASE_ATTACK, base_attack);
            data.set_fight_property(FIGHT_PROP_CUR_ATTACK, base_attack);

            data.set_fight_property(FIGHT_PROP_BASE_DEFENSE, base_defense);
            data.set_fight_property(FIGHT_PROP_CUR_DEFENSE, base_defense);

            data.set_fight_property(FIGHT_PROP_CHARGE_EFFICIENCY, config.charge_efficiency);
            data.set_fight_property(FIGHT_PROP_CRITICAL_HURT, config.critical_hurt);
            data.set_fight_property(FIGHT_PROP_CRITICAL, config.critical);

            data.set_fight_property(FIGHT_PROP_CUR_FIRE_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_CUR_ELEC_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_CUR_WATER_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_CUR_GRASS_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_CUR_WIND_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_CUR_ICE_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_CUR_ROCK_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_FIRE_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_ELEC_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_WATER_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_GRASS_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_WIND_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_ICE_ENERGY, 100.0);
            data.set_fight_property(FIGHT_PROP_MAX_ROCK_ENERGY, 100.0);

            self.avatars.push(data);
        }
    }
}

impl AvatarData {
    pub fn set_property(&mut self, prop_type: u32, value: i64) {
        if let Some(prop) = self.properties.iter_mut().find(|p| p.r#type == prop_type) {
            prop.val = value;
            prop.value = Some(prop_value::Value::Ival(value));
        } else {
            self.properties.push(PropValue {
                r#type: prop_type,
                val: value,
                value: Some(prop_value::Value::Ival(value)),
            });
        }
    }

    pub fn set_fight_property(&mut self, fight_prop_type: u32, value: f32) {
        if let Some(fight_prop) = self
            .fight_properties
            .iter_mut()
            .find(|fp| fp.prop_type == fight_prop_type)
        {
            fight_prop.prop_value = value;
            fight_prop.prop_type = fight_prop_type;
        } else {
            self.fight_properties.push(FightPropPair {
                prop_value: value,
                prop_type: fight_prop_type,
            });
        }
    }

    pub fn get_property(&self, prop_type: u32) -> i64 {
        if let Some(prop) = self.properties.iter().find(|p| p.r#type == prop_type) {
            prop.val
        } else {
            0
        }
    }

    pub fn get_fight_property(&self, fight_prop_type: u32) -> f32 {
        if let Some(fight_prop) = self
            .fight_properties
            .iter()
            .find(|fp| fp.prop_type == fight_prop_type)
        {
            fight_prop.prop_value
        } else {
            0.0
        }
    }
}

impl From<AvatarData> for AvatarInfo {
    fn from(value: AvatarData) -> Self {
        Self {
            guid: value.guid,
            avatar_id: value.avatar_id,
            skill_depot_id: value.skill_depot_id,
            wearing_flycloak_id: value.wearing_flycloak_id,
            costume_id: value.costume_id,
            born_time: value.born_time,
            life_state: 1,
            avatar_type: 1,
            fetter_info: Some(AvatarFetterInfo {
                exp_level: 1,
                ..Default::default()
            }),
            equip_guid_list: vec![value.weapon.guid],
            prop_map: value
                .properties
                .into_iter()
                .map(|prop| (prop.r#type, prop.clone()))
                .collect(),
            fight_prop_map: value
                .fight_properties
                .into_iter()
                .map(|pair| (pair.prop_type, pair.prop_value))
                .collect(),
            ..Default::default()
        }
    }
}
