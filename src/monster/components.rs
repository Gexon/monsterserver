// описание компонент монстра

use tinyecs::*;

use ::utility::map::Map;


// метка принадлежности к классу монстров.
pub struct MonsterClass;

impl Component for MonsterClass {}


// тут будем хранить все объекты на карте.
pub struct ViewMap {
    pub flora: Map<u8>,
}

impl Component for ViewMap {}