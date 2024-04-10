use proto::*;

use super::*;
use crate::{
    data,
    game::{constants, AvatarData},
    util,
};

pub struct SceneAvatar {
    data: AvatarData,
    weapon: SceneWeapon,
    entity_id: u32,
}

impl SceneAvatar {
    pub fn new(id: u32, data: AvatarData, weapon: SceneWeapon) -> Self {
        Self {
            data,
            weapon,
            entity_id: calc_entity_id(id, ProtEntityType::ProtEntityAvatar),
        }
    }

    #[allow(dead_code)]
    pub fn data(&self) -> AvatarData {
        self.data.clone()
    }

    pub fn avatar_guid(&self) -> u64 {
        self.data.guid
    }

    pub fn weapon_entity_id(&self) -> u32 {
        self.weapon.get_entity_id()
    }

    pub fn weapon_guid(&self) -> u64 {
        self.weapon.weapon_guid()
    }

    pub fn enter_info(&self) -> AvatarEnterSceneInfo {
        AvatarEnterSceneInfo {
            weapon_guid: self.weapon.weapon_guid(),
            avatar_guid: self.data.guid,
            avatar_entity_id: self.entity_id,
            weapon_entity_id: self.weapon.get_entity_id(),
            gipmpciehlj: Some(AbilitySyncStateInfo::default()),
            idkfhmfdofd: Some(AbilitySyncStateInfo::default()),
            ..Default::default()
        }
    }

    pub fn ability_control_block(&self) -> AbilityControlBlock {
        let mut control_block = AbilityControlBlock::default();

        let default_override_hash = util::hash_str("Default");
        data::COMMON_ABILITIES.iter().for_each(|a| {
            control_block.ability_embryo_list.push(AbilityEmbryo {
                ability_id: control_block.ability_embryo_list.len() as u32 + 1,
                ability_name_hash: util::hash_str(a),
                ability_override_name_hash: default_override_hash,
            })
        });

        data::CONFIG_COLLECTION
            .get_avatar_abilities(self.data.avatar_id)
            .abilities
            .iter()
            .for_each(|a| {
                control_block.ability_embryo_list.push(AbilityEmbryo {
                    ability_id: control_block.ability_embryo_list.len() as u32 + 1,
                    ability_name_hash: util::hash_str(&a.ability_name),
                    ability_override_name_hash: if a.ability_override.is_empty() {
                        default_override_hash
                    } else {
                        util::hash_str(&a.ability_override)
                    },
                })
            });

        control_block
    }
}

impl Entity for SceneAvatar {
    fn get_position(&self) -> Vector {
        constants::INITIAL_POS.clone()
    }

    fn get_entity_id(&self) -> u32 {
        self.entity_id
    }

    fn get_entity_type(&self) -> ProtEntityType {
        ProtEntityType::ProtEntityAvatar
    }

    fn info(&self) -> SceneEntityInfo {
        SceneEntityInfo {
            entity_type: self.get_entity_type().into(),
            entity_id: self.entity_id,
            motion_info: Some(MotionInfo {
                pos: Some(self.get_position()),
                rot: Some(Vector::default()),
                speed: Some(Vector::default()),
                ..Default::default()
            }),
            prop_list: self
                .data
                .properties
                .iter()
                .map(|p| PropPair {
                    r#type: p.r#type,
                    prop_value: Some(p.clone()),
                })
                .collect(),
            fight_prop_list: self.data.fight_properties.clone(),
            life_state: 1,
            animator_para_list: vec![AnimatorParameterValueInfoPair {
                animator_para: Some(Aglcgcoakfj::default()),
                name_id: 0,
            }],
            entity: Some(scene_entity_info::Entity::Avatar(SceneAvatarInfo {
                uid: 1337,
                peer_id: 1,
                avatar_id: self.data.avatar_id,
                guid: self.data.guid,
                equip_id_list: vec![self.weapon.weapon_id()],
                skill_depot_id: self.data.skill_depot_id,
                weapon: Some(self.weapon.scene_weapon_info()),
                team_resonance_list: vec![10301],
                wearing_flycloak_id: self.data.wearing_flycloak_id,
                born_time: self.data.born_time,
                excel_info: Some(AvatarExcelInfo::default()),
                ..Default::default()
            })),
            entity_client_data: Some(EntityClientData::default()),
            entity_authority_info: Some(EntityAuthorityInfo {
                ability_info: Some(AbilitySyncStateInfo::default()),
                born_pos: Some(Vector::default()),
                client_extra_info: Some(Acbnkemldob {
                    ampkijenfce: Some(Vector::default()),
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}
