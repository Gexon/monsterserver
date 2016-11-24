// работа с сетью.

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


#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterExport {
    id: i64,
    x: f32,
    y: f32,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterArray {
    entities: Vec<MonsterExport>
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
        aspect_all!(Modified)
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {
        for entity in entities {
            let monster_id = entity.get_component::<MonsterId>();
            let position = entity.get_component::<Position>();

            // создаем передаваюмую структуру с монстром.
            let monster_export = MonsterExport {
                id: monster_id.id, x: position.x, y: position.y,
            };
            let monster_array = MonsterArray {
                entities: vec![monster_export]
            };

            self.server_data.write(monster_array);
            println!("Послали монстра");
            entity.remove_component::<Modified>();
            entity.refresh();
        }
    }
}

/// сервер
pub struct Server {
    stream: TcpStream,
}

impl Server {
    fn write(&mut self, monster_array: MonsterArray) {
        let encoded: Vec<u8> = encode(&monster_array, SizeLimit::Infinite).unwrap();

        let len = encoded.len();
        let mut send_buf = [0u8; 8];
        BigEndian::write_u64(&mut send_buf, len as u64);

        let mut writer = BufWriter::new(&self.stream);
        let _ = writer.write(&send_buf);
        let _ = writer.write(&encoded);
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
        println!("Длина отправленных данных {}", len);
    }

    fn _read(&mut self) -> MonsterExport {
        // создаем читателя
        let mut reader = BufReader::new(&self.stream);
        // готовим вектор для примема размера входящих данных
        let mut buf_len = [0u8; 8];
        // принимаем данные
        let bytes = match reader.read(&mut buf_len) {
            Ok(n) => {
                let s = str::from_utf8(&buf_len[..]).unwrap();
                println!("Содержимое сообщения о длине входящих данных:{}, количество считанных байт:{}", s, n);
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
        debug!("Ожидаемая длина сообщения {}", msg_len);
        println!("Ожидаемая длина сообщения {}", msg_len);
        // подготавливаем вектор для принимаемых данных.
        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        unsafe { recv_buf.set_len(msg_len); }
        //let stream_ref = <TcpStream as Read>::by_ref(&self.stream);
        //match self.stream.take(msg_len as u64).read(&mut recv_buf) {
        // прием данных
        match reader.read(&mut buf_len) {
            Ok(n) => {
                debug!("CONN : считано {} байт", n);
                println!("CONN : считано {} байт", n);
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