

#[macro_use] extern crate tinyecs;

#[macro_use] extern crate slog;
extern crate slog_stream;
extern crate slog_stdlog;
#[macro_use] extern crate log;

extern crate bincode;
extern crate rustc_serialize;

use tinyecs::*;


mod server;
mod monster;
mod utility;


const  SERVER_IP: &'static str = "192.168.0.131";
//const  SERVER_IP: &'static str = "194.87.237.144";


fn main() {
    utility::init(); // запускаем логгер.
    let mut monster_world = World::new(); // создаем мир компонентной системы.
    server::init(&mut monster_world);    // инициализация сервера.
    monster::init(&mut monster_world);     // инициализация растений.
    loop {monster_world.update()}        // основной цикл ECS.
}
