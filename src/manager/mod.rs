// основные компоненты и системы, управляющего монстрами.

use tinyecs::*;
use time::{PreciseTime};

use ::utility::map::Map;
use ::utility::map::Size;
use ::manager::components::*;
use ::manager::systems::*;

pub mod components;
mod systems;

pub fn init(monster_world: &mut World) {
    // добавляем систему спавна.
    monster_world.set_system(SpawnSystem);
    monster_world.set_system(BioSystems { bios_time: PreciseTime::now() });

    {
        // вносим в этот мир немного земли
        let mut entity_manager = monster_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ClassManager);
        entity.add_component(WorldMap {
            monster: Map::new_empty(Size(140, 140), 0u8, 0u8),
            traces_map: Map::new_empty(Size(140, 140), 0u8, 0u8)
        });
        entity.add_component(WorldLastId { monster_id: 0 });
        entity.refresh();
    }
}