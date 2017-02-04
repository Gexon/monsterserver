// будем юзать Behaviour Tree AI

//Главный скрипт работает по системе выбор-условие-действие (selector-condition-action).

//Выбор — своеобразный аналог оператора switch() в языках программирования.
// В «теле» элемента выбора происходит выбор одного из заданных наборов действий
// в зависимости от условия.

//Условие — проверка истинности заданного условия.
// Используется в начале каждого набора действий внутри элемента выбора.
// Если условие истинно — выполняется данный набор действий и выбор завершается.
// Если нет — происходит переход к следующему набору действий

//Действие — скрипт, запускающий другой скрипт (действие) с заданными параметрами.
// *В BT AI существует понятие базовых действий.

/*
SelectorBegin('AI Role 1');
    SequenceBegin('Атака');
        //видим врага и знаем его id
        Condition(SeeEnemy && Enemy>0);
        //смомтрим на него
        Action( Action_LookAt, point_direction(x,y, Enemy.x, Enemy.y));
        //стреляем в сторону врага 2 раза
        Action( Action_Shoot, point_direction(x,y, Enemy.x, Enemy.y), 2);
        SelectorBegin('Подходим на оптимальное растояние');
            //или
            SequenceBegin('Враг слишком далеко');
                Condition(point_distance(x,y, Enemy.x, Enemy.y)>256);
                Action(Action_MoveTo, Enemy.x-lengthdir_x(128, direction), Enemy.y-lengthdir_y(128, direction), highSpeed);
            SequenceEnd();
            //или
            SequenceBegin('Враг слишком близко');
                Condition(point_distance(x,y, Enemy.x, Enemy.y)<64);
                //идем назад
                Action(Action_MoveTo, x-lengthdir_x(64, direction), y-lengthdir_y(64, direction), highSpeed);
            SequenceEnd();
            SequenceBegin('маневр');
                //иначе просто маневрируем, чтобы сложнее было попасть
                Action( Action_MoveTo, x+irandom_range(-64, 64), y+irandom_range(-64, 64), highSpeed);
            SequenceEnd();
        SelectorEnd();
        //стреляем в сторону врага 4 раза
        Action(Action_Shoot, point_direction(x,y, Enemy.x, Enemy.y), 2);
    SequenceEnd();
SelectorEnd();
*/

//Selector — оператор выбора набора действий
//Sequence — набор действий
//Condition — проверка условия
//Action — действие. вызов скрипта(первый аргумент) с параметрами (остальные аргументы)

/*
http://www.pvsm.ru/robototehnika/161885/print/
Узлы BT называют [10] задачами или поведениями. Каждая задача может иметь четыре состояния:

    «Успех», если задача выполнена успешно;
    - выкинуть нахер, заменитьт ошибкой. «Неудача», если условие не выполнено или задача, по какой-то причине, невыполнима;
    «В работе», если задача запущена в работу и ожидает завершения
    «Ошибка», если в программе возникает неизвестная ошибка.

 Результат работы любого узла всегда передается родительскому узлу, расположенному на уровень выше.
 Дерево просматривается с самого верхнего узла – корня. От него производится поиск в глубину начиная
 с левой ветви дерева. Если у одного узла есть несколько подзадач, они исполняются слева направо.

    Среди узлов выделяют следующие типы:
    -действие (action),
    -узел исполнения последовательности (sequence),
    -параллельный узел (parallel),
    -селектор (selector),
    -условие (condition),
    -инвертор (inverter).

 Действие представляет собой запись переменных или какое-либо движение.
 Узлы последовательностей поочередно исполняют поведения каждого дочернего узла до тех пор,
 пока один из них не выдаст значение «Неудача», «В работе» или «Ошибка».
 Если этого не произошло, возвращает значение «Успех».

 Узлы параллельных действий исполняют поведения дочерних узлов до тех пор,
 пока заданное количество из них не вернет статусы «Неудача» или «Успех».

 Селекторы поочередно исполняют поведения каждого дочернего узла до тех пор,
 пока один из них не выдаст значение «Успех», «В работе» или «Ошибка».
 Если этого не произошло, возвращает значение «Неудача».

 Условия содержат критерий, по которому определяется исход, и переменную.
 Например, условие «Есть ли в этой комнате человек?» перебирает все объекты в комнате
 и сравнивает их с переменной «Человек».

 Узлы инверсии выполняют функцию оператора NOT.
*/

use tinyecs::*;
use time::{PreciseTime, Duration};

use WORLD_SPEED;

use ::monster::components::MonsterClass;
use ::monster::components::MonsterAttributes;
use ::monster::components::SelectionTree;
use ::monster::components::BehaviourEvent;
use ::monster::components::BehaviourState;


/// Система восприятия
pub struct _PerceptionSystem;
// тут типа чекает окружение, и помечает объекты что попадают в поле зения.
impl System for _PerceptionSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn process_one(&mut self, _entity: &mut Entity) {
        // здесь тоже меняются события.
    }
}

/// Выбиральщик состояний дерева поведения
// используя код программатора SelectionTree, переключает состояния.
pub struct SelectorSystem;

impl System for SelectorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, SelectionTree, BehaviourEvent, BehaviourState)
    }

    //    fn process_no_entities(&mut self) {
    //        println!("instaced buffer render system must work, but no entities!");
    //    }
    //    fn process_no_data(&mut self) {
    //        println!("instaced buffer render system must work, but no data!");
    //    }

    fn process_one(&mut self, entity: &mut Entity) {
        let mut selection_tree = entity.get_component::<SelectionTree>();
        let mut behaviour_state = entity.get_component::<BehaviourState>(); // состояние
        let behaviour_event = entity.get_component::<BehaviourEvent>(); // события

        let len = selection_tree.selector.len();
        if len > 0 {
            // ткущий узел.
            if selection_tree.curr_selector < 0i32 {
                selection_tree.curr_selector = 0i32;
                println!("ошибка/инициализация текущего указателя, теперь он {}", 0i32);
            } else {
                /*event, state
                let sel = vec![[6, 2], [5, 1]];*/
                let index: usize = selection_tree.curr_selector as usize;
                let curr_cell = selection_tree.selector[index]; //[6, 2]
                let v_event = curr_cell[0];
                let v_state = curr_cell[1];
                // проверить нет ли ошибки в селекторе/программаторе. или первый запуск/инициализация.
                let curr_event = behaviour_event.event; // считываем текущий событие/event
                if curr_event == v_event {
                    // меняем состояние, на соответствующее.
                    behaviour_state.state = v_state;
                    println!("обнаружено событие {}", v_event);
                    println!("переключаю состояние на {}", v_state);
                    // сдвигаем curr_selector, переходим к сл. ячейке.
                    let shl: i32 = (len - 1) as i32;
                    if selection_tree.curr_selector < shl { selection_tree.curr_selector += 1; } else {
                        selection_tree.curr_selector = 0;
                    }
                }
            }
        }
    }
}

/// Активатор. Приводит в действие.
// считывает состояние и выполняет его, либо продолжает выполнение.
// система поведения.
pub struct BehaviorSystem {
    pub behavior_time: PreciseTime,
}

impl System for BehaviorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, BehaviourState)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        // смотрит текущее состояние и выполняет действие.
        // тут заставляем монстра ходить, пить, искать.
        //    0.  Инициализация, ошибка.
        //    1.  Сон. Монстр ждет, в этот момент с ним ничего не происходит.
        //    2.  Бодрствование. Случайное перемещение по полигону.
        //    3.  Поиск пищи.
        //    4.  Поиск воды.
        //    5.  Прием пищи.
        //    6.  Прием воды.
        //    7.  Перемещение к цели.
        //    8.  Проверка достижения цели.
        if self.behavior_time.to(PreciseTime::now()) > Duration::seconds(5 * WORLD_SPEED) {
            let behaviour_state = entity.get_component::<BehaviourState>(); // состояние
            match behaviour_state.state {
                1 => {
                    println!("...сплю zzz...");
                },
                2 => {
                    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
                    println!("...я хожу туда-сюда...");
                },
                _ => {},
            }
            // фиксируем текущее время
            self.behavior_time = PreciseTime::now();
        }
    }
}

/// Генерация событий
// Создаем события, проверяем параметры.
pub struct EventSystem {
    pub event_time: PreciseTime,
    pub event_last: u32,
    // 0 - инициализация/ошибка
}

impl System for EventSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(BehaviourEvent, MonsterAttributes)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        //    0.  Инициализация, ошибка.
        //    1.  Обнаружена еда.
        //    2.  Обнаружена вода.
        //    3.  Наступил голод.
        //    4.  Наступила жажда.
        //    5.  Утомился.
        //    6.  Нет событий.
        //    7.  Монстр насытился.
        //    8.  Монстр напился.
        if self.event_time.to(PreciseTime::now()) > Duration::seconds(2 * WORLD_SPEED) {
            let mut behaviour_event = entity.get_component::<BehaviourEvent>(); // события
            let monster_attr = entity.get_component::<MonsterAttributes>(); // события
            if behaviour_event.event == 0 {
                // проверяем ошибки/инициализация
                behaviour_event.event = 6;
                println!("ошибка/инициализация текущего события, теперь он {}", 6);
            } else if self.event_last != behaviour_event.event {
                if monster_attr.power < 950 {
                    behaviour_event.event = 5;
                    println!("...я устал, мне нужно поспать...");
                }
                if monster_attr.power > 990 {
                    behaviour_event.event = 6;
                    println!("...я свеж и полон сил!...");
                }
            }

            // фиксируем текущее время
            self.event_time = PreciseTime::now();
        }
    }
}


