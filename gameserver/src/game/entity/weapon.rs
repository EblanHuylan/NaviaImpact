use proto::*;

use super::*;
use crate::game::WeaponData;

pub struct SceneWeapon {
    data: WeaponData,
    entity_id: u32,
}

impl SceneWeapon {
    pub fn new(id: u32, data: WeaponData) -> Self {
        Self {
            entity_id: calc_entity_id(id, ProtEntityType::ProtEntityWeapon),
            data,
        }
    }

    pub fn weapon_id(&self) -> u32 {
        self.data.weapon_id
    }

    pub fn weapon_guid(&self) -> u64 {
        self.data.guid
    }

    pub fn scene_weapon_info(&self) -> SceneWeaponInfo {
        SceneWeaponInfo {
            entity_id: self.entity_id,
            gadget_id: 50000000 + self.data.weapon_id,
            item_id: self.data.weapon_id,
            guid: self.data.guid,
            level: 1,
            promote_level: 0,
            ability_info: Some(AbilitySyncStateInfo::default()),
            renderer_changed_info: Some(EntityRendererChangedInfo::default()),
            ..Default::default()
        }
    }
}

impl Entity for SceneWeapon {
    fn get_position(&self) -> Vector {
        Vector::default()
    }

    fn set_position(&mut self, _pos: Vector) {}

    fn get_entity_id(&self) -> u32 {
        self.entity_id
    }

    fn get_entity_type(&self) -> ProtEntityType {
        ProtEntityType::ProtEntityWeapon
    }

    fn info(&self) -> SceneEntityInfo {
        SceneEntityInfo::default()
    }
}
