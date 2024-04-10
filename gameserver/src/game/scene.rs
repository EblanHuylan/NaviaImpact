use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::Result;
use proto::*;

use super::{
    entity::{Entity, SceneAvatar, SceneWeapon},
    AvatarData,
};
use crate::net::PlayerSession;

pub struct Scene {
    id: u32,
    entity_id_counter: AtomicU32,
    pub avatars: Vec<SceneAvatar>,
    pub cur_entity_id: u32,
}

impl Scene {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            entity_id_counter: AtomicU32::new(0),
            avatars: Vec::new(),
            cur_entity_id: 0,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    fn next_entity_id(&self) -> u32 {
        self.entity_id_counter.fetch_add(1, Ordering::SeqCst) + 1
    }

    pub fn add_avatar(&mut self, data: AvatarData) {
        let weapon = SceneWeapon::new(self.next_entity_id(), data.weapon.clone());

        self.avatars
            .push(SceneAvatar::new(self.next_entity_id(), data, weapon));
    }

    pub async fn sync_scene_team(&self, session: &PlayerSession) -> Result<()> {
        let scene_team_update = SceneTeamUpdateNotify {
            scene_team_avatar_list: self
                .avatars
                .iter()
                .map(|a| SceneTeamAvatar {
                    player_uid: session.player_uid(),
                    scene_entity_info: Some(a.info()),
                    weapon_entity_id: a.weapon_entity_id(),
                    weapon_guid: a.weapon_guid(),
                    entity_id: a.get_entity_id(),
                    avatar_guid: a.avatar_guid(),
                    ability_control_block: Some(a.ability_control_block()),
                    gipmpciehlj: Some(AbilitySyncStateInfo::default()),
                    idkfhmfdofd: Some(AbilitySyncStateInfo::default()),
                    gjepahinlhl: true,
                    scene_id: self.id,
                    agojdhammpn: true,
                    bjjbdmaoepn: true,
                    ..Default::default()
                })
                .collect(),
        };

        session
            .send(SCENE_TEAM_UPDATE_NOTIFY, scene_team_update)
            .await
    }
}
