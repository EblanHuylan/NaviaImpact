use proto::{ProtEntityType, SceneEntityInfo, Vector};

mod avatar;
mod weapon;

pub use avatar::SceneAvatar;
pub use weapon::SceneWeapon;

pub trait Entity {
    fn get_position(&self) -> Vector;
    fn get_entity_id(&self) -> u32;
    fn get_entity_type(&self) -> ProtEntityType;
    fn info(&self) -> SceneEntityInfo;
}

pub fn calc_entity_id(id: u32, entity_type: ProtEntityType) -> u32 {
    ((entity_type as u32) << 24) + id
}
