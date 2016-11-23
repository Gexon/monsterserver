// работа с сетью.

use tinyecs::*;

use std::net::TcpStream;
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;
use std::str;

use byteorder::{ByteOrder, BigEndian};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

use SERVER_IP;

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterExport {
    id: u64,
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
        aspect_all!(ServerClass)
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {
        for entity in entities {
            self.server_data.write();
            println!("Послали монстра");
            //entity.remove_component::<ServerClass>();
            //entity.refresh();
        }
    }
}

/// сервер
pub struct Server {
    stream: TcpStream,
}

impl Server {
    fn write(&mut self) {
        // @AlexNav73 - спс за ссылку и помощь в освоении этой сериализации!
        let monster_export = MonsterExport {
            id: 0, x: 50f32, y: 50f32,
        };

        let monster_export2 = MonsterExport {
            id: 1, x: 51f32, y: 50f32,
        };

        let monster_array = MonsterArray {
            entities: vec![monster_export, monster_export2]
        };

        let encoded: Vec<u8> = encode(&monster_array, SizeLimit::Infinite).unwrap();

        let len = encoded.len();
        let mut send_buf = [0u8; 8];
        BigEndian::write_u64(&mut send_buf, len as u64);

        let mut writer = BufWriter::new(&self.stream);
        let _ = writer.write(&send_buf);
        let _ = writer.write(&encoded);
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
        println!("Длина отправленных данных {}", len);
        let s = str::from_utf8(&send_buf[..]).unwrap();
        println!("Содержимое сообщения о длине отправленных данных:{:?}", s);
        let s = str::from_utf8(&encoded[..]).unwrap();
        println!("Содержимое отправленных данных:{:?}", s);
    }

    fn read(&mut self) -> MonsterExport {
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