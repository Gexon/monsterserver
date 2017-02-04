// системы менеджера

use tinyecs::*;
use time::{PreciseTime, Duration};

use WORLD_SPEED;

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
                //world_map.monster[target_point] = 1;

                // проверяем наличие заданных объектов.
                // создаем объект Монстр.
                let entity_object = world.entity_manager.create_entity();
                entity_object.add_component(Name { name: spawn_point.name.to_string() });
                entity_object.add_component(Position { x: spawn_point.x, y: spawn_point.y });
                entity_object.add_component(MonsterClass);
                entity_object.add_component(Modified); // произошли изменения монстра.
                entity_object.add_component(MonsterId { id: last_id.monster_id });
                entity_object.add_component(SelectionTree::new());
                entity_object.add_component(BehaviourState { state: 0 });
                entity_object.add_component(BehaviourEvent { event: 0 });
                entity_object.add_component(MonsterAttributes { power: 1000 });
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

/// Bio Systems, будем умершвлять тут монстра.
// пересчет характеристик, значений жизнедеятельности монстра.
pub struct BioSystems {
    pub bios_time: PreciseTime,
}

impl System for BioSystems {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterAttributes, BehaviourState)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        if self.bios_time.to(PreciseTime::now()) > Duration::seconds(2 * WORLD_SPEED) {
            let mut monster_attr = entity.get_component::<MonsterAttributes>();
            let behaviour_state = entity.get_component::<BehaviourState>(); // состояние
            if monster_attr.power > 0 {
                if behaviour_state.state == 1 {
                    monster_attr.power += 1;
                } else {
                    monster_attr.power -= 1;
                }
                println!("power {}", monster_attr.power);
            }
            // фиксируем текущее время
            self.bios_time = PreciseTime::now();
        }
    }
}