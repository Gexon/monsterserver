// описание компонент монстра

use tinyecs::*;

use ::utility::map::Map;


// метка принадлежности к классу монстров.
pub struct MonsterClass;

impl Component for MonsterClass {}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {}

// тут будем хранить все объекты на карте.
pub struct ViewMap {
    pub flora: Map<u8>,
}

impl Component for ViewMap {}