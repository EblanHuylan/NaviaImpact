use super::*;

pub async fn on_mark_map_req(session: &PlayerSession, body: &MarkMapReq) -> Result<()> {
    if let Some(mark) = body.mark.as_ref() {
        let mut scene = session.context.scene.borrow_mut();
        let mut pos = mark.pos.clone().unwrap();
        pos.y = 500.0;

        scene.teleport(&pos);
        scene.enter(session, EnterType::EnterJump).await?;
    }

    session.send(MARK_MAP_RSP, MarkMapRsp::default()).await
}

pub async fn on_get_scene_area_req(session: &PlayerSession, body: &GetSceneAreaReq) -> Result<()> {
    let rsp = GetSceneAreaRsp {
        area_id_list: (0..50).collect(),
        scene_id: body.scene_id,
        ..Default::default()
    };

    session.send(GET_SCENE_AREA_RSP, rsp).await
}
