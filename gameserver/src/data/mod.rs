mod configs;
mod excels;

use std::collections::HashMap;

pub use configs::*;
pub use excels::*;
use lazy_static::lazy_static;
use serde_json::from_str;

pub fn init_assets() {
    tracing::info!("Loaded {} excel tables", EXCEL_COLLECTION.table_count());
    let _config_collection = &*CONFIG_COLLECTION;
}

lazy_static! {
    pub static ref EXCEL_COLLECTION: ExcelCollection = ExcelCollection::new();
    pub static ref CONFIG_COLLECTION: GameConfigCollection = GameConfigCollection::new();
}

pub struct ExcelCollection {
    pub avatar_configs: Vec<AvatarExcelConfig>,
}

pub struct GameConfigCollection {
    avatar_ability_configs: HashMap<u32, AvatarAbilityConfig>,
}

impl ExcelCollection {
    fn new() -> Self {
        Self {
            avatar_configs: from_str(&load_asset(
                "assets/ExcelBinOutput/AvatarExcelConfigData.json",
            ))
            .unwrap(),
        }
    }

    pub fn table_count(&self) -> usize {
        1
    }
}

impl GameConfigCollection {
    fn new() -> Self {
        let mut avatar_ability_configs = HashMap::new();

        for config in EXCEL_COLLECTION.avatar_configs.iter() {
            if let Some(avatar_name) = config.side_icon_name.split('_').last() {
                avatar_ability_configs.insert(
                    config.id,
                    from_str(&load_asset(
                        format!("assets/BinOutput/Ability/ConfigAvatar_{avatar_name}.json")
                            .as_str(),
                    ))
                    .unwrap(),
                );
            }
        }

        Self {
            avatar_ability_configs,
        }
    }

    pub fn get_avatar_abilities(&self, avatar_id: u32) -> &AvatarAbilityConfig {
        self.avatar_ability_configs.get(&avatar_id).unwrap()
    }
}

fn load_asset(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
