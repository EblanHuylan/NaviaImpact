use std::sync::Arc;

use anyhow::Result;
use atomic_refcell::AtomicRefCell;

use super::{components::GameTimeComponent, PlayerInfo, Scene};
use crate::net::PlayerSession;

pub struct GameContext {
    pub player: Arc<AtomicRefCell<PlayerInfo>>,
    pub scene: Arc<AtomicRefCell<Scene>>,
    pub time: Arc<AtomicRefCell<GameTimeComponent>>,
}

impl GameContext {
    pub fn new(player: Arc<AtomicRefCell<PlayerInfo>>) -> Self {
        let scene = Arc::new(AtomicRefCell::new(Scene::new(3)));

        Self {
            player: player.clone(),
            scene: scene.clone(),
            time: Arc::new(AtomicRefCell::new(GameTimeComponent::new(scene))),
        }
    }

    pub async fn init_components(&self, session: &PlayerSession) -> Result<()> {
        self.time.borrow().init(session).await?;

        Ok(())
    }
}
