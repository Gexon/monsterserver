// работа с сетью.

use tinyecs::*;

use std::net::TcpStream;
use std::io::{BufWriter, BufReader};
use std::io::prelude::*;


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

        let stream = TcpStream::connect(&*address).unwrap();
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
        self.server_data.write();
        println!("Послали монстра");
        for entity in entities {
            entity.remove_component::<ServerClass>();
            entity.refresh();
        }
    }
}

/// сервер
pub struct Server {
    stream: TcpStream,
}

impl Server {
    fn write(&mut self) {
        //        let world = World {
        //            entities: vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]
        //        };

        // @AlexNav73 - спс за ссылку и помощь в освоении этой сериализации!
        let monster_export = MonsterExport {
            id: 0, x: 50f32, y: 50f32,
        };
        let encoded: Vec<u8> = encode(&monster_export, SizeLimit::Infinite).unwrap();

        let mut writer = BufWriter::new(&self.stream);
        let _ = writer.write(&encoded);
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    }

    fn _read(&mut self) -> MonsterExport {
        let mut buf = vec![];
        let mut reader = BufReader::new(&self.stream);
        reader.read(&mut buf).unwrap();

        decode(&buf[..]).unwrap()
    }
}