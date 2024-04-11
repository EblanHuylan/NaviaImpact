use std::collections::HashMap;

use common::encryption::{rsa_decrypt, rsa_encrypt, rsa_sign};
use rand::RngCore;

use super::*;
use crate::{
    data::EXCEL_COLLECTION, game::{constants, entity::Entity, prop_types::*}, util
};

pub async fn on_get_player_token_req(
    session: &PlayerSession,
    body: &GetPlayerTokenReq,
) -> Result<()> {
    let client_rand_key = rsa_decrypt(&rbase64::decode(&body.client_rand_key)?);

    let client_rand_key = u64::from_be_bytes(client_rand_key.try_into().unwrap());
    let server_rand_key = rand::thread_rng().next_u64();

    let rand_key_bytes = server_rand_key.to_be_bytes();
    let encrypted_rand_key = rsa_encrypt(&rand_key_bytes);
    let rand_key_sign = rsa_sign(&rand_key_bytes);

    let rsp = GetPlayerTokenRsp {
        server_rand_key: rbase64::encode(&encrypted_rand_key),
        sign: rbase64::encode(&rand_key_sign),
        country_code: String::from("RU"),
        uid: session.player_uid(),
    };

    session.send(GET_PLAYER_TOKEN_RSP, rsp).await?;
    session.set_xorpad(util::generate_xorpad(client_rand_key ^ server_rand_key));

    Ok(())
}

pub async fn on_player_login_req(session: &PlayerSession, _body: &PlayerLoginReq) -> Result<()> {
    session.player_info_mut().unlock_all_avatars();

    let login_time_ms = util::cur_timestamp_ms();
    let login_time = login_time_ms / 1000;

    let props = [
        (PROP_PLAYER_LEVEL, 5),
        (PROP_IS_FLYABLE, 1),
        (PROP_IS_TRANSFERABLE, 1),
        (PROP_MAX_STAMINA, 10000),
        (PROP_CUR_PERSIST_STAMINA, 10000),
        (PROP_IS_SPRING_AUTO_USE, 1),
        (PROP_SPRING_AUTO_USE_PERCENT, 50),
    ];

    session
        .send(
            PLAYER_DATA_NOTIFY,
            PlayerDataNotify {
                nick_name: String::from("NaviaImpact"),
                prop_map: HashMap::from(props.map(|(prop_type, prop_value)| {
                    (
                        prop_type,
                        PropValue {
                            r#type: prop_type,
                            val: prop_value,
                            value: Some(prop_value::Value::Ival(prop_value)),
                        },
                    )
                })),
                ..Default::default()
            },
        )
        .await?;

    session
        .send(
            OPEN_STATE_UPDATE_NOTIFY,
            OpenStateUpdateNotify {
                open_state_map: EXCEL_COLLECTION.open_state_configs
                    .iter()
                    .map(|op| (op.id, 1))
                    .collect(),
            },
        )
        .await?;

    let player_info = session.player_info();
    let avatar_data_notify = AvatarDataNotify {
        avatar_list: player_info
            .avatars
            .iter()
            .map(|a| Into::<AvatarInfo>::into(a.clone()))
            .collect(),
        cur_avatar_team_id: 1,
        choose_avatar_guid: 228,
        avatar_team_map: HashMap::from([(
            1,
            AvatarTeam {
                avatar_guid_list: vec![
                    player_info
                        .avatars
                        .iter()
                        .find(|a| a.avatar_id == 10000091)
                        .unwrap()
                        .guid,
                ],
                team_name: String::from("LogicTeam"),
            },
        )]),
        ..Default::default()
    };

    session.send(AVATAR_DATA_NOTIFY, avatar_data_notify).await?;

    {
        let mut scene = session.context.scene.borrow_mut();
        let player = session.player_info();

        scene.add_avatar(
            player
                .avatars
                .iter()
                .find(|a| a.avatar_id == 10000091)
                .unwrap()
                .clone(),
        );

        scene.cur_entity_id = scene.avatars.first().unwrap().get_entity_id()
    }

    let scene = session.context.scene.borrow();
    let scene_id = scene.id();
    let player_uid = session.player_uid();
    let enter_scene = PlayerEnterSceneNotify {
        scene_begin_time: 7313,
        scene_id,
        enter_scene_token: 15771,
        scene_transaction: format!("{scene_id}-{player_uid}-{login_time}-179398"),
        scene_tag_id_list: Vec::new(),
        pos: Some(constants::INITIAL_POS.clone()),
        prev_pos: Some(Vector::default()),
        target_uid: player_uid,
        r#type: EnterType::EnterSelf.into(),
        ..Default::default()
    };

    session.send(PLAYER_ENTER_SCENE_NOTIFY, enter_scene).await?;

    session
        .send(
            PLAYER_LOGIN_RSP,
            PlayerLoginRsp {
                game_biz: String::from("hk4e_global"),
                country_code: String::from("RU"),
                res_version_config: Some(ResVersionConfig::default()),
            },
        )
        .await
}

pub async fn on_ping_req(session: &PlayerSession, body: &PingReq) -> Result<()> {
    let rsp = PingRsp {
        client_time: body.client_time,
        seq: body.seq,
        ..Default::default()
    };

    session.send(PING_RSP, rsp).await
}
