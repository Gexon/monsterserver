// будем юзать Behaviour Tree AI

//*Главный скрипт работает по системе выбор-условие-действие (selector-condition-action).

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


//use tinyecs::*;