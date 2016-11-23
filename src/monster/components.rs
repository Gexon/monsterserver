// описание компонент монстра

use tinyecs::*;

use ::utility::map::Map;


/// метка принадлежности к классу монстров.
pub struct MonsterClass;

impl Component for MonsterClass {}


/// имя монстра
pub struct Name {
    pub name: String
}

impl Component for Name {}


/// уникальный номер монстра
pub struct MonsterId {
    pub id: i64,
}

impl Component for MonsterId {}


/// характеристики монстра и его текущее состояние
pub struct _MonsterState {
    //pub state: i32,
    //pub growth_time: PreciseTime,
    //pub reproduction_time: PreciseTime,
    //pub dead: i32,
}

impl Component for _MonsterState {}


/// тут будем хранить все объекты на карте.
pub struct _MonsterMaps {
    pub view_map: Map<u8>,
    pub foods_map: Map<u8>,
    pub waters_map: Map<u8>,
}

impl Component for _MonsterMaps {}