// основные компоненты и системы, общие для монстра, буду хранить тут.

use tinyecs::*;

use ::utility::map::Map;
use ::utility::map::Size;
use ::monster::components::*;

pub mod components;
pub mod systems;

pub fn init(monster_world: &mut World) {
    // добавляем в мир систему спавна.
    //monster_world.set_system(SpawnSystem);

    {
        // вносим в этот мир немного земли
        let mut entity_manager = monster_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(MonsterClass);
        entity.add_component(ViewMap { flora: Map::new_empty(Size(140, 140), 0u8, 0u8) });
        entity.refresh();
    }
}