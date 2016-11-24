// основные компоненты и системы, общие для монстра, буду хранить тут.

use tinyecs::*;

use ::manager::components::SpawnPoint;
pub mod components;
pub mod systems;


/// инициализация. создаем первого монстра.
pub fn init(monster_world: &mut World) {
    // добавляем в мир систему роста растений.
    //dk_world.set_system(PlantGrowthSystem);

    for count in 0..10 {
        // поручаем спавнеру, засумонить в наш мир первого монстра!
        // создаем спавнер
        let mut entity_manager = monster_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        let delta: f32 = count as f32;
        entity_spawner.add_component(SpawnPoint { name: "monster", x: 20f32 + delta, y: 20f32 + delta });
        entity_spawner.refresh();
        break;
    }
}