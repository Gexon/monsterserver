// работа с сетью.

use tinyecs::*;

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::slice;
use std::mem;

use SERVER_IP;


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

//        if let Some(ref data) = stream {
//            info!("Монстер-сервер запущен. Подключен к главному серверу.");
//            println!("Монстер-сервер запущен. Подключен к главному серверу.");
//        }

//        let stream = match TcpStream::connect(&*address) {
//            Ok(data) => {
//                info!("Монстер-сервер запущен. Подключен к главному серверу.");
//                println!("Монстер-сервер запущен. Подключен к главному серверу.");
//                data
//            },
//            Err(e) => {
//                println!("Монстер-сервер. Ошибка открытия порта: {}", e);
//                None
//            },
//        };



        let mut server = Server {
            stream: BufReader::new(stream),
            writer: BufWriter::new(stream),
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

    }
}

/// сервер
pub struct Server {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Server {
    pub fn send_data(&mut self, data: &str) {
        let size_dat = data.len();

        //превращаем размер в байты
        let size: usize = size_dat;
        let const_size: *const usize = &size;
        let bp: *const u8 = const_size as *const _;
        let bs: &[u8] = unsafe {
            slice::from_raw_parts(
                bp,
                mem::size_of::<usize>()
            )
        };

        println!("Размер данных answer {}", data.len());
        println!("Содержимое size_dat {}", size_dat);
        println!("Размер байтмассива bs {}", bs.len()); //8

        let _ = self.writer.write(bs);   // шлем 8 байт размер данных.
        let _ = self.writer.write(data.as_bytes());
        self.writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    }
}