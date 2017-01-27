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


/// Сосотояние монстра, для Behaviour Tree
pub struct BehaviourGlobalState {
    //    Всего 2 глобальных состояния:
    //    1.  Размеренное. Монстр спокоен.
    //    2.  Тревожное. Потеря здоровья.
    pub state: u32,
}

impl Component for BehaviourGlobalState {}

/// Сосотояние монстра, для Behaviour Tree
pub struct BehaviourState {
    //    Всего 6 состояний:
    //    1.  Сон. Монстр ждет, в этот момент с ним ничего не происходит.
    //    2.  Бодрствование. Случайное перемещение по полигону.
    //    3.  Поиск пищи.
    //    4.  Поиск воды.
    //    5.  Прием пищи.
    //    6.  Прием воды.
    pub state: u32,
}

impl Component for BehaviourState {}

/// Событий происходящее с монстром, для Behaviour Tree
pub struct BehaviourEvent {
    //    Всего 6 событий:
    //    1.  Обнаружена еда.
    //    2.  Обнаружена вода.
    //    3.  Наступил голод.
    //    4.  Наступила жажда.
    //    5.  Выспался.
    //    6.  Нет событий.
    //    7.  Монстр насытился.
    //    8.  Монстр напился.
    pub event: u32,
}

impl Component for BehaviourEvent {}