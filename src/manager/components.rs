// компоненты менеджера

use tinyecs::*;

use ::utility::map::Map;


/// просто метка для менеджера.
pub struct ClassManager;

impl Component for ClassManager {}


/// тут будем хранить все объекты на карте.
pub struct WorldMap {
    // мы намеренно не будем хранить растения и прочие объекты в глобальной карте,
    // т.к. для монстра эти объекты будут храниться в его памяти.
    pub monster: Map<u8>,
    // храним монстров, для взаимодействия между друг-другом.
    // возможно их стоит перенести на основной сервер.
    pub traces_map: Map<u8>,
    // следы, для поиска пути.
}

impl Component for WorldMap {}


/// текущая позиция монстра
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {}


/// координаты спавнируемого монстра
pub struct SpawnPoint {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Component for SpawnPoint {}


/// храним последний уникальный номер монстра
pub struct WorldLastId {
    pub monster_id: i64,
}

impl Component for WorldLastId {}