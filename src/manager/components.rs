// компоненты менеджера

use tinyecs::*;

use ::utility::map::Map;

// тут будем хранить все объекты на карте.
pub struct WorldMap {
    pub flora: Map<u8>,
}

// текущая позиция монстра
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// координаты спавнируемого монстра
pub struct SpawnPoint {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Component for SpawnPoint {}