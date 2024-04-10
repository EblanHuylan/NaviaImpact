use super::*;
use crate::game::entity::Entity;

pub async fn on_enter_scene_ready_req(
    session: &PlayerSession,
    body: &EnterSceneReadyReq,
) -> Result<()> {
    let scene = session.context.scene.borrow();

    let enter_scene_peer = EnterScenePeerNotify {
        dest_scene_id: scene.id(),
        enter_scene_token: body.enter_scene_token,
        peer_id: 1,
        host_peer_id: 1,
    };

    session
        .send(ENTER_SCENE_PEER_NOTIFY, enter_scene_peer)
        .await?;

    let enter_scene_ready = EnterSceneReadyRsp {
        enter_scene_token: body.enter_scene_token,
    };

    session.send(ENTER_SCENE_READY_RSP, enter_scene_ready).await
}

pub async fn on_scene_init_finish_req(
    session: &PlayerSession,
    body: &SceneInitFinishReq,
) -> Result<()> {
    session.context.init_components(session).await?;

    let scene = session.context.scene.borrow();

    let enter_scene_info = PlayerEnterSceneInfoNotify {
        cur_avatar_entity_id: scene.avatars.get(0).unwrap().get_entity_id(),
        enter_scene_token: body.enter_scene_token,
        mp_level_entity_info: Some(MpLevelEntityInfo {
            entity_id: 184549377,
            ability_info: Some(AbilitySyncStateInfo::default()),
            authority_peer_id: 1,
        }),
        team_enter_info: Some(TeamEnterSceneInfo {
            team_entity_id: 150994946,
            ability_control_block: Some(AbilityControlBlock::default()),
            team_ability_info: Some(AbilitySyncStateInfo::default()),
        }),
        avatar_enter_info: scene.avatars.iter().map(|a| a.enter_info()).collect(),
    };

    session
        .send(PLAYER_ENTER_SCENE_INFO_NOTIFY, enter_scene_info)
        .await?;

    scene.sync_scene_team(session).await?;

    session
        .send(
            SCENE_INIT_FINISH_RSP,
            SceneInitFinishRsp {
                enter_scene_token: body.enter_scene_token,
            },
        )
        .await
}

pub async fn on_enter_scene_done_req(
    session: &PlayerSession,
    body: &EnterSceneDoneReq,
) -> Result<()> {
    let scene = session.context.scene.borrow();
    let scene_entity_appear = SceneEntityAppearNotify {
        appear_type: VisionType::VisionBorn.into(),
        entity_list: scene.avatars.iter().map(|a| a.info()).collect(),
        ..Default::default()
    };

    session
        .send(SCENE_ENTITY_APPEAR_NOTIFY, scene_entity_appear)
        .await?;

    session
        .send(
            ENTER_SCENE_DONE_RSP,
            EnterSceneDoneRsp {
                enter_scene_token: body.enter_scene_token,
            },
        )
        .await
}

pub async fn on_post_enter_scene_req(
    session: &PlayerSession,
    body: &PostEnterSceneReq,
) -> Result<()> {
    session
        .send(
            POST_ENTER_SCENE_RSP,
            PostEnterSceneRsp {
                enter_scene_token: body.enter_scene_token,
            },
        )
        .await
}

pub async fn on_evt_avatar_sit_down_req(
    session: &PlayerSession,
    body: &EvtAvatarSitDownReq,
) -> Result<()> {
    session
        .send(
            EVT_AVATAR_SIT_DOWN_RSP,
            EvtAvatarSitDownRsp {
                retcode: 0,
                chair_id: body.chair_id,
                entity_id: session.context.scene.borrow().cur_entity_id,
                perform_id: body.perform_id,
                position: body.position.clone(),
            },
        )
        .await
}

pub async fn on_evt_avatar_stand_up_notify(
    session: &PlayerSession,
    body: &EvtAvatarStandUpNotify,
) -> Result<()> {
    session.send(EVT_AVATAR_STAND_UP_NOTIFY, body.clone()).await
}
