// системы менеджера

use tinyecs::*;

use ::manager::components::*;
use ::monster::components::*;
use ::utility::map::Point;

/// Система создает объекты в мире.
pub struct SpawnSystem;

impl System for SpawnSystem {
    // Обрабатываем сущности содержащие компоненты "SpawnPoint", "Position"
    // Аспект - список сущностей, содержащих выбранные компоненты.
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnPoint)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassManager]]
    }

    // обработчик, вызывается при update, process_all - 1 раз вызывается.
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let manager = data.unwrap_entity();
        let mut last_id = manager.get_component::<WorldLastId>();
        let mut world_map = manager.get_component::<WorldMap>();

        // перебираем все сущности
        for entity in entities {
            // берем компонент "Точка спавна/spawn_point"
            let spawn_point = entity.get_component::<SpawnPoint>();

            // проверяем свободно ли место спавна.
            let target_point: Point = Point(spawn_point.x.trunc() as i32, spawn_point.y.trunc() as i32); // Casting

            //println!("Пробуем создать сущность: x {}, y {}", target_point.0, target_point.1);
            if world_map.monster[target_point] == 0 {
                world_map.monster[target_point] = 1;
                world_map.monster[target_point] = 1;

                // проверяем наличие заданных объектов.
                // создаем объект Монстр.
                let entity_object = world.entity_manager.create_entity();
                entity_object.add_component(Name { name: spawn_point.name.to_string() });
                entity_object.add_component(Position { x: spawn_point.x, y: spawn_point.y });
                entity_object.add_component(MonsterClass);
                //entity_object.add_component(Growth);
                //entity_object.add_component(Replication); // требуется репликация.
                entity_object.add_component(MonsterId { id: last_id.monster_id });
                entity_object.refresh();
                let monster_id = entity_object.get_component::<MonsterId>();
                println!("Создаем сущность {} {}", spawn_point.name.to_string(), monster_id.id);
                last_id.monster_id += 1;
            }

            entity.remove_component::<SpawnPoint>(); // удаляем компонент "Точка спавна/spawn_point"
            entity.delete();
        }
    }
}