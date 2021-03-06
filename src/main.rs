extern crate byteorder;
extern crate time;

#[macro_use] extern crate tinyecs;

#[macro_use] extern crate slog;
extern crate slog_stream;
extern crate slog_stdlog;
#[macro_use] extern crate log;

extern crate bincode;
extern crate rustc_serialize;

use tinyecs::*;


mod utility;
mod server;
mod manager;
mod monster;

const SERVER_IP: &'static str = "192.168.0.131";
//const  SERVER_IP: &'static str = "194.87.237.144";
static WORLD_SPEED: i64 = 1;

fn main() {
    utility::init(); // запускаем логгер.
    let mut monster_world = World::new(); // создаем мир компонентной системы.
    server::init(&mut monster_world);     // инициализация сетевой части.
    manager::init(&mut monster_world);    // инициализация менеджера монстров.
    monster::init(&mut monster_world);    // инициализация монстров.
    loop { monster_world.update() }       // основной цикл ECS.
}
