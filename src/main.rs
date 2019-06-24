#![feature(proc_macro_hygiene, decl_macro)]
#![feature(rustc_private)]

use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::env;
use std::sync::mpsc;
use std::str::{self, from_utf8};
use std::io::prelude::*;
use std::io::{self, Cursor, Write, BufReader, BufRead, Read};
use std::fs::{self, File};
use std::path::Path;
use std::ops::Deref;
use std::ffi::{CStr, CString};

//use libc::c_char;

mod db;
use db::redis::RedisConnection;

use sha2::{Sha256, Digest};

use fs_extra;

//mod zip;
use mwish_c_encoding;


use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::PathChildrenCache;
use protobuf::{self, Message, ProtobufResult};

use uuid::Uuid;
use sysinfo::{NetworkExt, System, SystemExt, DiskExt};
use eventual::Timer;

use zip;

use serde::{Serialize, Deserialize};
use data_encoding::{HEXUPPER, HEXLOWER};

use rocket_contrib::json::{Json, JsonValue};
use redis;
use redis::RedisResult;
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
const TMP_DATA_DIR: &'static str = "data/temp";

#[derive(Deserialize)]
struct PostBody {
    size: u64
}


/// Store the uuid pair of
#[derive(Deserialize, Serialize)]
struct RedisUuidMessage {
    /// Size of the file
    size: u64,
    /// Unique id to identify the file
    uid: String,
    /// Hash value of the file
    file_hash: String
}

fn exists_datafile_with_name<T: AsRef<str>>(filename :T)-> bool {
    // TODO: build with more specific name
    // TODO: learn how to handle Rust string
    let new_name = String::from(DATA_DIR.clone().to_string() + filename.as_ref() + ".dat");
    let path = Path::new(&new_name);
    let ret = path.exists() && path.is_file();
    ret
}

/// 上传之前先查询 uuid
///
/// If Ok return target uuid, which provides the next operations
/// Get size from body
///
/// Return 400 when meeting duplicate
#[post("/temp/<file_hash>", data = "<post_body>")]
fn temp_uuid_request(post_body: Json<PostBody>, connection: RedisConnection, file_hash: String)->Result<String, Custom<String>> {
    // TODO: make clear the way to handle string
    if exists_datafile_with_name(&file_hash) {
        return Err(Custom(Status::BadRequest, "duplicate hash".to_string()));
    }
    let post_body: PostBody = post_body.into_inner();
    // get size
    let size = post_body.size;

    // create new uuid string
    let uid = Uuid::new_v4().to_string();
    let message = RedisUuidMessage {size, uid: uid.clone(), file_hash};
//    let message = serde_json::to_string(&message).map_err(|e| Err(Custom(Status::InternalServerError, "error occurs when changing message to string".to_string())));
    let message = serde_json::to_string(&message).unwrap();
    // set 60 second for expire
    let _: () = connection.set_ex(uid.clone(), message, 12000).unwrap();

    Ok(uid)
}

#[put("/temp/<uid>")]
fn move_to_persistence(uid: String, connection: RedisConnection) -> Result<(), Custom<String>> {
    let s: RedisResult<String> = connection.get(&uid);
    if !s.is_ok() {
        return Err(Custom(Status::NotFound, format!("uuid {} not found", uid.clone())));
    }
    let s = s.unwrap();
    // change to uuid message
    let s: RedisUuidMessage = serde_json::from_str(&s).unwrap();

    // TODO: use concat to optimize these code

    // old path
    let old_name: String = TMP_DATA_DIR.clone().to_string() + &"/".to_string() + &s.uid;
    let new_name: String = DATA_DIR.clone().to_string() +  &"/".to_string() + &s.file_hash + ".zip";
    println!("old_name: {}", old_name);
    let old_name = CString::new(old_name).unwrap();
    let new_name = CString::new(new_name).unwrap();

    unsafe {
        mwish_c_encoding::oss_compress(old_name.as_ptr(), new_name.as_ptr());
    }
//    let old_path = Path::new(&old_name);
//    let new_path = Path::new(&new_name);
//
//    fs_extra::file::move_file(old_path, new_path, &fs_extra::file::CopyOptions::new());

    Ok(())
}

#[patch("/temp/<uid>", data = "<data>")]
fn upload_temp_data(uid: String, data: Data, connection: RedisConnection)->Result<(), Custom<String>> {
    // throw unimplemented
    let s: RedisResult<String> = connection.get(&uid);
    if !s.is_ok() {
        return Err(Custom(Status::NotFound, format!("uuid {} not found", uid.clone())));
    }
    let s = s.unwrap();
    // change to uuid message
    let s: RedisUuidMessage = serde_json::from_str(&s).unwrap();

//    let new_name = String::from(DATA_DIR.clone().to_string() + filename.as_ref() + ".dat");
    let new_name: String = TMP_DATA_DIR.clone().to_string() + "/" + &s.uid;
    let path = Path::new(&new_name);
    // create file with path
    // TODO: add some logic lock to the file
//    let created_file = File::create(path).map_err(|io_error| Custom(Status::InternalServerError, "create file error"))?;
    // Load stream to file
    data.stream_to_file(path);

    const BUFFER_SIZE: usize = 1024;
    if let Ok(mut file) = fs::File::open(&path) {
        let mut sh = Sha256::default();
        let mut buffer = [0u8; BUFFER_SIZE];
        loop {

            let n = match file.read(&mut buffer) {
                Ok(n) => n,
                Err(_) => {
                    return Err(Custom(Status::InternalServerError, "Error when checking file".to_string()));
                },
            };
            sh.input(&buffer[..n]);
            if n == 0 || n < BUFFER_SIZE {
                break;
            }
        }
        // TODO: handle this
        let shas = sh.result();
        println!("Upper {}; Lower {}", HEXLOWER.encode(shas.as_slice()), HEXUPPER.encode(shas.as_slice()));
        if HEXLOWER.encode(shas.as_slice()) == s.file_hash {
            return Ok(());
        } else {
            return Err(Custom(Status::BadRequest, "Different hash when uploading file".to_string()));
        }

    } else {
        return Err(Custom(Status::InternalServerError, "Error when opening file".to_string()));
    }
    // 存储成功
    Ok(())
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

#[get("/hash/<hash>")]
fn handle_hash_get(hash: String) {
//    println!("你妈死了");
    let path = String::from("data/") + &hash;
    let file_path = Path::new(&path);
}

#[get("/data/<uid>")]
fn handle_get(uid: String)->Result<rocket::response::NamedFile, Custom<String>> {
    println!("Get uid {}", uid);

    let path = String::from("data/") + &uid;
    let file_path = Path::new(&path);

    if !file_path.exists() && file_path.is_file() {
        // maybe zip
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
        .mount("/", routes![temp_uuid_request])
        .mount("/", routes![upload_temp_data])
        .launch();
}
