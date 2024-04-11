use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::Result;
use proto::*;
use rand::RngCore;

use super::{
    constants,
    entity::{Entity, SceneAvatar, SceneWeapon},
    AvatarData,
};
use crate::{net::PlayerSession, util};

pub struct Scene {
    id: u32,
    entity_id_counter: AtomicU32,
    last_player_pos: Vector,
    pub avatars: Vec<SceneAvatar>,
    pub cur_entity_id: u32,
}

impl Scene {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            last_player_pos: constants::INITIAL_POS.clone(),
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

        self.avatars.push(SceneAvatar::new(
            self.next_entity_id(),
            data,
            weapon,
            self.cur_player_pos(),
        ));
    }

    pub fn cur_player_pos(&self) -> Vector {
        if let Some(avatar) = self
            .avatars
            .iter()
            .find(|a| a.get_entity_id() == self.cur_entity_id)
        {
            avatar.get_position()
        } else {
            self.last_player_pos.clone()
        }
    }

    pub fn teleport(&mut self, to_pos: &Vector) {
        self.last_player_pos = to_pos.clone();

        for avatar in self.avatars.iter_mut() {
            avatar.set_position(self.last_player_pos.clone())
        }
    }

    pub async fn enter(&mut self, session: &PlayerSession, enter_type: EnterType) -> Result<()> {
        let begin_time_ms = util::cur_timestamp_ms();
        let begin_time = begin_time_ms / 1000;
        let scene_id = self.id;
        let player_uid = session.player_uid();

        let enter_scene = PlayerEnterSceneNotify {
            scene_begin_time: begin_time_ms,
            scene_id,
            enter_scene_token: rand::thread_rng().next_u32() % 65535,
            scene_transaction: format!("{scene_id}-{player_uid}-{begin_time}-179398"),
            pos: Some(self.last_player_pos.clone()),
            prev_pos: Some(Vector::default()),
            target_uid: player_uid,
            r#type: enter_type.into(),
            ..Default::default()
        };

        session.send(PLAYER_ENTER_SCENE_NOTIFY, enter_scene).await
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
