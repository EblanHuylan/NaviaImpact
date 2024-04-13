use std::collections::HashMap;

use super::*;
use crate::game::entity::{Entity, SceneWeapon};

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

pub async fn on_avatar_change_costume_req(
    session: &PlayerSession,
    body: &AvatarChangeCostumeReq,
) -> Result<()> {
    let mut player = session.player_info_mut();
    let Some(avatar) = player
        .avatars
        .iter_mut()
        .find(|a| a.guid == body.avatar_guid)
    else {
        return session
            .send(
                AVATAR_CHANGE_COSTUME_RSP,
                AvatarChangeCostumeRsp {
                    retcode: -1,
                    ..Default::default()
                },
            )
            .await;
    };

    avatar.costume_id = body.costume_id;

    let mut scene = session.context.scene.borrow_mut();
    if let Some(avatar_entity) = scene
        .avatars
        .iter_mut()
        .find(|a| a.avatar_guid() == body.avatar_guid)
    {
        avatar_entity.update_data(avatar);
        session
            .send(
                AVATAR_CHANGE_COSTUME_NOTIFY,
                AvatarChangeCostumeNotify {
                    entity_info: Some(avatar_entity.info()),
                },
            )
            .await?;
    }

    session
        .send(
            AVATAR_CHANGE_COSTUME_RSP,
            AvatarChangeCostumeRsp {
                avatar_guid: body.avatar_guid,
                costume_id: body.costume_id,
                retcode: 0,
            },
        )
        .await
}

pub async fn on_avatar_wear_flycloak_req(
    session: &PlayerSession,
    body: &AvatarWearFlycloakReq,
) -> Result<()> {
    let mut player = session.player_info_mut();
    let Some(avatar) = player
        .avatars
        .iter_mut()
        .find(|a| a.guid == body.avatar_guid)
    else {
        return session
            .send(
                AVATAR_WEAR_FLYCLOAK_RSP,
                AvatarWearFlycloakRsp {
                    retcode: -1,
                    ..Default::default()
                },
            )
            .await;
    };

    avatar.wearing_flycloak_id = body.flycloak_id;

    let mut scene = session.context.scene.borrow_mut();
    if let Some(avatar_entity) = scene
        .avatars
        .iter_mut()
        .find(|a| a.avatar_guid() == body.avatar_guid)
    {
        avatar_entity.update_data(avatar);
    }

    session
        .send(
            AVATAR_FLYCLOAK_CHANGE_NOTIFY,
            AvatarFlycloakChangeNotify {
                avatar_guid: body.avatar_guid,
                flycloak_id: body.flycloak_id,
            },
        )
        .await?;

    session
        .send(
            AVATAR_WEAR_FLYCLOAK_RSP,
            AvatarWearFlycloakRsp {
                retcode: 0,
                avatar_guid: body.avatar_guid,
                flycloak_id: body.flycloak_id,
            },
        )
        .await
}

pub async fn on_wear_equip_req(session: &PlayerSession, body: &WearEquipReq) -> Result<()> {
    let mut player = session.player_info_mut();

    let weapon = player
        .weapons
        .iter()
        .find(|w| w.guid == body.equip_guid)
        .map(|w| w.clone());

    let avatar = player
        .avatars
        .iter_mut()
        .find(|a| a.guid == body.avatar_guid);

    let (Some(weapon), Some(avatar)) = (weapon, avatar) else {
        return session
            .send(
                WEAR_EQUIP_RSP,
                WearEquipRsp {
                    retcode: -1,
                    ..Default::default()
                },
            )
            .await;
    };

    avatar.weapon = weapon;

    session
        .send(
            AVATAR_EQUIP_CHANGE_NOTIFY,
            AvatarEquipChangeNotify {
                avatar_guid: body.avatar_guid,
                equip_guid: body.equip_guid,
                item_id: avatar.weapon.weapon_id,
                equip_type: 6,
                weapon: Some(SceneWeapon::new(0, avatar.weapon.clone()).scene_weapon_info()),
                reliquary: None,
            },
        )
        .await?;

    let mut scene = session.context.scene.borrow_mut();
    if let Some(avatar_entity) = scene
        .avatars
        .iter_mut()
        .find(|a| a.avatar_guid() == body.avatar_guid)
    {
        avatar_entity.update_data(&avatar);
        avatar_entity.change_weapon(avatar.weapon.clone());
    }

    session
        .send(
            WEAR_EQUIP_RSP,
            WearEquipRsp {
                retcode: 0,
                equip_guid: body.equip_guid,
                avatar_guid: body.avatar_guid,
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
