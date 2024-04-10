use std::sync::Arc;

use anyhow::Result;
use atomic_refcell::AtomicRefCell;
use proto::*;

use crate::{game::Scene, net::PlayerSession};

pub const INITIAL_TIME: u32 = 7300;

pub struct GameTimeComponent {
    scene: Arc<AtomicRefCell<Scene>>,
    time: u32,
}

impl GameTimeComponent {
    pub fn new(scene: Arc<AtomicRefCell<Scene>>) -> Self {
        Self {
            scene,
            time: INITIAL_TIME,
        }
    }

    pub async fn init(&self, session: &PlayerSession) -> Result<()> {
        session
            .send(
                SCENE_TIME_NOTIFY,
                SceneTimeNotify {
                    scene_id: self.scene.borrow().id(),
                    ..Default::default()
                },
            )
            .await?;

        self.sync(session).await
    }

    pub fn set_time(&mut self, time: u32) {
        self.time = time;
    }

    pub async fn sync(&self, session: &PlayerSession) -> Result<()> {
        session
            .send(
                PLAYER_GAME_TIME_NOTIFY,
                PlayerGameTimeNotify {
                    uid: session.player_uid(),
                    game_time: self.time,
                    ..Default::default()
                },
            )
            .await
    }
}
