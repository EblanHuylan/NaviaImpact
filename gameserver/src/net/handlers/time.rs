use super::*;

pub async fn on_change_game_time_req(
    session: &PlayerSession,
    body: &ChangeGameTimeReq,
) -> Result<()> {
    let mut time = session.context.time.borrow_mut();
    time.set_time(body.game_time);
    time.sync(session).await?;

    session
        .send(
            CHANGE_GAME_TIME_RSP,
            ChangeGameTimeRsp {
                retcode: 0,
                game_time: body.game_time,
                extra_days: body.extra_days,
            },
        )
        .await
}
