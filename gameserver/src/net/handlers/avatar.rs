use std::collections::HashMap;

use super::*;
use crate::game::entity::Entity;

pub async fn on_change_avatar_req(session: &PlayerSession, body: &ChangeAvatarReq) -> Result<()> {
    let mut scene = session.context.scene.borrow_mut();

    let scene_entity_appear = SceneEntityAppearNotify {
        appear_type: VisionType::VisionReplace.into(),
        entity_list: scene
            .avatars
            .iter()
            .filter(|a| a.avatar_guid() == body.guid)
            .map(|a| a.info())
            .collect(),
        ..Default::default()
    };

    scene.cur_entity_id = scene_entity_appear.entity_list.first().unwrap().entity_id;

    session
        .send(SCENE_ENTITY_APPEAR_NOTIFY, scene_entity_appear)
        .await?;

    session
        .send(
            CHANGE_AVATAR_RSP,
            ChangeAvatarRsp {
                cur_avatar_guid: body.guid,
            },
        )
        .await
}

pub async fn on_set_up_avatar_team_req(
    session: &PlayerSession,
    body: &SetUpAvatarTeamReq,
) -> Result<()> {
    session
        .send(
            AVATAR_TEAM_UPDATE_NOTIFY,
            AvatarTeamUpdateNotify {
                avatar_team_map: HashMap::from([(
                    body.team_id,
                    AvatarTeam {
                        avatar_guid_list: body.avatar_team_guid_list.clone(),
                        ..Default::default()
                    },
                )]),
                ..Default::default()
            },
        )
        .await?;

    let player = session.player_info();
    let mut scene = session.context.scene.borrow_mut();

    scene.avatars.clear();
    player
        .avatars
        .iter()
        .filter(|a| body.avatar_team_guid_list.contains(&a.guid))
        .for_each(|data| scene.add_avatar(data.clone()));

    scene.sync_scene_team(session).await?;

    let scene_entity_appear = SceneEntityAppearNotify {
        appear_type: VisionType::VisionBorn.into(),
        entity_list: vec![scene.avatars.first().unwrap().info()],
        ..Default::default()
    };
    scene.cur_entity_id = scene_entity_appear.entity_list.first().unwrap().entity_id;

    session
        .send(SCENE_ENTITY_APPEAR_NOTIFY, scene_entity_appear)
        .await?;

    session
        .send(
            SET_UP_AVATAR_TEAM_RSP,
            SetUpAvatarTeamRsp {
                retcode: 0,
                team_id: body.team_id,
                cur_avatar_guid: body.cur_avatar_guid,
                avatar_team_guid_list: body.avatar_team_guid_list.clone(),
            },
        )
        .await
}
