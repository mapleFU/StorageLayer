#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::env;
use std::sync::mpsc;
use std::str::{self, from_utf8};
use std::io::{self, Cursor, Write};
use std::fs::{self, File};
use std::path::Path;
use std::ops::Deref;

mod db;
use db::redis::RedisConnection;

use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::PathChildrenCache;
use protobuf::{self, Message, ProtobufResult};

use uuid::Uuid;
use sysinfo::{NetworkExt, System, SystemExt, DiskExt};
use eventual::Timer;

use serde::{Serialize, Deserialize};

use rocket_contrib::json::{Json, JsonValue};
use redis;
use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands as RedisCommands;
use rocket::{self, get, routes, put, post, patch};
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use rocket::Data;
use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket::config::{Config as RocketConfig, Environment as RocketEnvironment};
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData,
                                 MultipartFormDataField, FileField, TextField, RawField};


use oss_storage_layer::zk::{Server, self};


impl Deref for RedisConnection {
    type Target = r2d2::PooledConnection<RedisConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
//        info!("{:?}", e)
        println!("{:?}", e)
    }
}

fn zk_server_urls() -> String {
    let key = "ZOOKEEPER_SERVERS";
    match env::var(key) {
        Ok(val) => val,
        Err(_) => "maplewish.cn:2181".to_string(),
    }
}


const ZKDIR: &'static str = "/fs";
const DATA_DIR: &'static str = "data";


#[derive(Deserialize)]
struct PostBody {
    size: u64
}
/// 上传之前先查询 uuid
///
/// If Ok return target uuid, which provides the next operations
/// Get size from body
#[post("/temp/<file_hash>", data = "<post_body>")]
fn temp_upload(post_body: Json<PostBody>, connection: RedisConnection, file_hash: String)->Result<String, Custom<String>> {
    let post_body: PostBody = post_body.into_inner();

    let size = post_body.size;
//    if let None = size {
//        return Err(Custom(Status::ExpectationFailed, "Require header for size.".to_string()));
//    }
//    let size = size.unwrap();
    // throw unimplemented
    unimplemented!();
    // create new uuid string
    let current_uuid = Uuid::new_v4();


    Ok(current_uuid.to_string())
}

#[patch("/temp/<uid>", data = "<data>")]
fn move_to_persistence(uid: String, data: Data)->Result<(), Custom<String>> {
    // throw unimplemented
    unimplemented!();
    Err(Custom(Status::NotImplemented, "Not Implemented".to_string()))
}

#[put("/data/<uid>", data = "<data>")]
fn handle_post(uid: String, data: Data)->Result<(), Custom<String>> {
    println!("Request {}", uid);
//    println!("Got a put request raw data");
    data.stream_to_file(format!("data/{}", uid)).map(|n| n.to_string());
    Ok(())
}


#[put("/", data = "<data>")]
fn handle_multi(cont_type: &ContentType, data: Data) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>>{
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary
    println!("Something begin in this request");
    if !cont_type.is_form_data() {
        println!("Got a put request raw data");
        data.stream_to_file("data/demo").map(|n| n.to_string());
        return Ok(Stream::from(Cursor::new(Vec::from("nmsl"))));
    }

    let (_, boundary) = cont_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
        || Custom(
            Status::BadRequest,
            "`Content-Type: multipart/form-data` boundary param not provided".into()
        )
    )?;
    let mut mp = Multipart::with_body(data.open(), boundary);

    mp.foreach_entry(|mut e| {
        println!("{}", str::from_utf8(e.headers.name.as_bytes()).unwrap());
        println!("{:?}", e.data.save().with_dir(DATA_DIR));
    });
    Ok(Stream::from(Cursor::new(Vec::from("nmsl"))))
}

#[get("/data/<uid>")]
fn handle_get(uid: String)->Result<rocket::response::NamedFile, Custom<String>> {
    println!("Get uid {}", uid);

    let path = String::from("data/") + &uid;
    let file_path = Path::new(&path);

    if !file_path.exists() && file_path.is_file() {
        return Err(Custom(Status::NotFound, "NotFound".to_string()))
    }
    // TODO: unuse unwrap
    Ok(rocket::response::NamedFile::open(file_path).unwrap())
}



fn zn_conn() -> Server {
    // connection to zookeeper
    let mut zk_pb = zk::Server::new();
    zk_pb.host = match env::var("ZMQ_HOST") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(),
    };

    zk_pb.zmq_tcp_port = match env::var("ZMQ_HOST") {
        Ok(val) => val,
        Err(_) => "11234".to_string(),
    };

    zk_pb.http_port = match env::var("HTTP_PORT") {
        Ok(val) => val,
        Err(_) => "8000".to_string()
    };

    zk_pb
}


fn main() {


    let mut sys = System::new();

    let zk_urls = zk_server_urls();
    println!("connecting to {}", zk_urls);
    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15),
                                LoggingWatcher).unwrap();


    let ctx = zmq::Context::new();

    let mut zmq_socket = ctx.socket(zmq::SUB).unwrap();
    let mut zmq_push_socket = ctx.socket(zmq::PUSH).unwrap();

    thread::spawn(move || {
        if let Err(e) = zmq_socket.connect("tcp://127.0.0.1:11234") {
            println!("{:?}", e);
            panic!();
        }
        if let Err(e) = zmq_push_socket.connect("tcp://127.0.0.1:5558") {
            println!("{:?}", e);
            panic!();
        }
        zmq_socket.set_subscribe("object ".as_bytes());
        loop {
            let mut message = zmq::Message::new().unwrap();
            let rs = zmq_socket.recv(&mut message, zmq::DONTWAIT);


            if let Err(r) = rs {
                if r != zmq::Error::EAGAIN {
                    println!("{:?}", r);
                }
                thread::sleep(Duration::from_millis(1));
                continue;
            }
            match message.as_str() {
                None => println!("Recv None"),
                Some(s) => {
                    let cur_path = String::from("data/") + s;
                    if Path::new(&cur_path).exists() {
                        println!("Receive request for path {}", cur_path);
                        let zn_msg = zn_conn();
                        let zn_msg: Vec<u8> =  Vec::from(zn_msg.write_to_bytes().unwrap());
                        zmq_push_socket.send(zn_msg.as_slice(), zmq::DONTWAIT);
                    }
                }
            };
        }
    });

    // connection to zookeeper
    let mut zk_pb = zn_conn();

    let http_port_i32 = zk_pb.http_port.clone().parse::<u16>().unwrap();

//    protobuf::parse_from_bytes()
    let mut server_data: Vec<u8> = Vec::from(zk_pb.write_to_bytes().unwrap());

    // call once
    if let Err(e) = zk.create(ZKDIR, Vec::new(), Acl::open_unsafe().clone(), CreateMode::Persistent) {
        println!("{:?}", e);
    } else {
        println!("Created");
    }

    // create current node
    // TODO: set acl for this
    let node = zk.create("/fs/Node",
                         server_data,
                         Acl::open_unsafe().clone(),
                         CreateMode::EphemeralSequential).unwrap();

    // spawn the loop for zookeeper
    thread::spawn(move || {
        let timer = Timer::new();
        let ticks = timer.interval_ms(1000).iter();
        for _ in ticks {
            // execute code once a second, send results via `tx`
//            zk_conn.available_space =
            let (mut total_space, mut available_space) = (0, 0);
            for disk in sys.get_disks() {
                // TODO: make clear if it's ok
//                println!("{} {} mount on {:?} ", disk.get_total_space(), disk.get_available_space(), disk.get_mount_point());
                if disk.get_total_space() > total_space {
                    total_space = disk.get_total_space();
                    available_space = disk.get_available_space();
                }
            }
            zk_pb.available_space = available_space;
            zk_pb.total_space = total_space;
            // send to zk
            let mut server_data: Vec<u8> = Vec::from(zk_pb.write_to_bytes().unwrap());
            zk.set_data(&node, server_data, None);
        }
    });


    let config = RocketConfig::build(RocketEnvironment::Staging)
        .address("0.0.0.0")
        .port(http_port_i32)
        .finalize().unwrap();

    let redis_address = match env::var("REDIS_ADDRESS") {
        Ok(val) => val,
        Err(_) => "redis://127.0.0.1/".to_string(),
    };

    // unwrap redis result
    let redis_client = redis::Client::open(redis_address.as_str()).unwrap();
    let manager = RedisConnectionManager::new(redis_address.as_str()).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();

    rocket::custom(config)
        .manage(pool)
        .mount("/", routes![handle_multi])
        .mount("/",  routes![handle_post])
        .mount("/", routes![handle_get])
        .mount("/", routes![move_to_persistence])
        .mount("/", routes![temp_upload])
        .launch();
}
