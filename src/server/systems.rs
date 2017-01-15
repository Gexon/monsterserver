// работа с сетью

use tinyecs::*;

use std::net::TcpStream;
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;
use std::str;

use byteorder::{ByteOrder, BigEndian};

use bincode::SizeLimit; // @AlexNav73 - спс за ссылку и помощь в освоении этой сериализации!
use bincode::rustc_serialize::{encode, decode};

use SERVER_IP;


use ::manager::components::Position;
use ::manager::components::Modified;
use ::monster::components::MonsterId;
use ::monster::components::MonsterClass;

// шлем отсюда
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterExport {
    p_type: u8,
    id: i64,
    x: f32,
    y: f32,
}

// принимаем сюда
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterImport {
    p_type: u8,
    id: u64,
    damage: u64,
}

// массив отправляемых данных
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterArrayExport {
    entities: Vec<MonsterExport>
}

// массив принимаемых данных
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterArrayImport {
    entities: Vec<MonsterImport>
}


pub struct ServerClass;

impl Component for ServerClass {}


/// Система по обработке сети.
pub struct ServerSystem {
    server_data: Server,
}

impl ServerSystem {
    pub fn new() -> ServerSystem {
        let hostname: &str = SERVER_IP;
        let port: &str = "6658";
        let address = format!("{}:{}", hostname, port);

        let stream = TcpStream::connect(&*address).expect("Ошибка подключения к основному серверу.");
        info!("Монстер-сервер запущен. Подключен к главному серверу.");
        println!("Монстер-сервер запущен. Подключен к главному серверу.");

        let server = Server {
            stream: stream,
        };

        ServerSystem {
            server_data: server,
        }
    }
}

// работа с сетью. обмен данными с главным сервером.


impl System for ServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(ServerClass)
    }

    // получение разных аспектов
    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![MonsterClass]]
    }

    fn process_all(&mut self, _entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        // Принимаем с основного сервера данные.
        let monster_array_import = self.server_data.read();
        // обрабатываем полученные данные
        if !monster_array_import.entities.is_empty() {
            let monster_entities = monster_array_import.entities;
            for monster in monster_entities {
                let in_monster: MonsterImport = monster;
                if in_monster.p_type == 0 {
                    println!("Приняли idle"); //TODO переделать, иначе будет работать со скоростью основного сервера.
                } else {
                    println!("Приняли монстра id {}, damage {}", in_monster.id, in_monster.damage);
                }
            }
        } else { println!("От Монстра-сервера пришли пустые данные."); }

        let mut send_monsters: bool = false;
        let monsters = data.unwrap_all();
        for monster in monsters {
            if monster.has_component::<Modified>() {
                let monster_id = monster.get_component::<MonsterId>();
                let position = monster.get_component::<Position>();

                // создаем передаваюмую структуру с монстром.
                let monster_export = MonsterExport {
                    p_type: 1, id: monster_id.id, x: position.x, y: position.y,
                };
                let monster_array = MonsterArrayExport {
                    entities: vec![monster_export]
                };
                // Шлем на основной сервер данные.
                self.server_data.write(monster_array);
                send_monsters = true;
                println!("Послали монстра {}", monster_id.id);

                monster.remove_component::<Modified>();
                monster.refresh();
            }
        }
        if !send_monsters {
            // создаем передаваюмую структуру с монстром.
            let monster_export = MonsterExport {
                p_type: 0, id: 0, x: 0f32, y: 0f32,
            };
            let monster_array = MonsterArrayExport {
                entities: vec![monster_export]
            };
            // Шлем на основной сервер данные.
            self.server_data.write(monster_array);
            println!("Послали idle");
        }
    }
}

/// сервер
pub struct Server {
    stream: TcpStream,
}

impl Server {
    fn write(&mut self, monster_array: MonsterArrayExport) {
        let encoded: Vec<u8> = encode(&monster_array, SizeLimit::Infinite).unwrap();

        let len = encoded.len();
        let mut send_buf = [0u8; 8];
        BigEndian::write_u64(&mut send_buf, len as u64);

        let mut writer = BufWriter::new(&self.stream);
        let _ = writer.write(&send_buf);
        let _ = writer.write(&encoded);
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
        //println!("Длина отправленных данных {}", len);
    }

    fn read(&mut self) -> MonsterArrayImport {
        // создаем читателя
        let mut reader = BufReader::new(&self.stream);
        // готовим вектор для примема размера входящих данных
        let mut buf_len = [0u8; 8];
        // принимаем данные
        let bytes = match reader.read(&mut buf_len) {
            Ok(n) => {
                //let s = str::from_utf8(&buf_len[..]).unwrap();
                //println!("Содержимое сообщения о длине входящих данных:{}, количество считанных байт:{}", s, n);
                n
            },
            Err(_) => {
                0
            }
        };
        if bytes < 8 {
            warn!("Ошибка. Сообщение о длине входящих данных меньше 8 байт и равно: {} bytes", bytes);
            println!("Ошибка. Сообщение о длине входящих данных меньше 8 байт и равно: {} bytes", bytes);
        }
        // превращаем в нормальный вид длину входящих данных.
        let msg_len = BigEndian::read_u64(buf_len.as_ref());
        let msg_len = msg_len as usize;
        //debug!("Ожидаемая длина сообщения {}", msg_len);
        //println!("Ожидаемая длина сообщения {}", msg_len);
        // подготавливаем вектор для принимаемых данных.
        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        unsafe { recv_buf.set_len(msg_len); }
        //let stream_ref = <TcpStream as Read>::by_ref(&self.stream);
        //match self.stream.take(msg_len as u64).read(&mut recv_buf) {
        // прием данных
        match reader.read(&mut recv_buf) {
            Ok(n) => {
                //debug!("CONN : считано {} байт", n);
                //println!("CONN : считано {} байт", n);
                if n < msg_len as usize {
                    println!("Не осилил достаточно байт");
                }
                decode(&recv_buf[..]).unwrap()
            }
            Err(_) => {
                panic!("Неудалось считать буфер для сокета");
            }
        }
    }
}