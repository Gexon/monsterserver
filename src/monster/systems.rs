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

use ::monster::components::MonsterClass;
use ::monster::components::SelectionTree;
use ::monster::components::BehaviourEvent;
use ::monster::components::BehaviourState;


/// система восприятия
pub struct PerceptionSystem;
// тут типа чекает окружение, и помечает объекты что попадают в поле зения.
impl System for PerceptionSystem {
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

    fn process_one(&mut self, entity: &mut Entity) {
        let selection_tree = entity.get_component::<SelectionTree>();
        let behaviour_state = entity.get_component::<BehaviourState>(); // состояние
        let behaviour_event = entity.get_component::<BehaviourEvent>(); // события
        let mut state = behaviour_state.state;
        let selector = &selection_tree.selector;
        let mut curr_selector = selection_tree.curr_selector; // узел
        let len = selector.len();
        if len > 0 {
            // ткущий узел.
            if curr_selector > -1i32 {
                curr_selector = 0i32;
            } else {
                /*event, state
                let sel = vec![[6, 2], [2, 6]];*/
                let index: usize = curr_selector as usize;
                let curr_cell = selector[index]; //[6, 2]
                let v_event = curr_cell[0];
                let v_state = curr_cell[1];
                // проверить нет ли ошибки в селекторе/программаторе. или первый запуск/инициализация.
                let mut curr_event = behaviour_event.event; // считываем текущий событие/event
                if curr_event == v_event {
                    // меняем состояние, на соответствующее.
                    state = v_state;
                } else if curr_event == 0 { curr_event = 6 } // проверяем ошибки/инициализация
                // сдвигаем curr_selector, переходим к сл. ячейке.
                let shl: i32 = (len -1) as i32;
                if curr_selector < shl { curr_selector += 1; } else { curr_selector = 0; }
            }
        }
    }
}

/// Активатор. Приводит в действие.
// считывает состояние и выполняет его, либо продолжает выполнение.
// система поведения.
pub struct BehaviorSystem {}

impl System for BehaviorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn process_one(&mut self, _entity: &mut Entity) {
        // смотрит текущее состояние и выполняет действие.
    }
}

/// Обработка событий
// отлавливаем события и оповещаем об этом монстра.
pub struct EventSystem {}

impl System for EventSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn process_one(&mut self, _entity: &mut Entity) {}
}