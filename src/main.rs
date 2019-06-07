#![feature(proc_macro_hygiene, decl_macro)]

use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::env;
use std::sync::mpsc;
use std::str::{self, from_utf8};
use std::io::{self, Cursor, Write};
use std::fs::{self, File};


use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::PathChildrenCache;
use protobuf::{self, Message, ProtobufResult};
use sysinfo::{NetworkExt, System, SystemExt, DiskExt};
use eventual::Timer;
use rocket::{self, get, routes, put};
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;
use rocket::Data;
use rocket::http::{ContentType, Status};
use rocket::response::Stream;
use rocket::response::status::Custom;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData,
                                 MultipartFormDataField, FileField, TextField, RawField};


use oss_storage_layer::zk::{Server, self};


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
        Err(_) => "localhost:2181".to_string(),
    }
}


const ZKDIR: &'static str = "/fs";
const DATA_DIR: &'static str = "data";

#[put("/", data = "<data>")]
fn handle_multi(cont_type: &ContentType, data: Data) -> Result<Stream<Cursor<Vec<u8>>>, Custom<String>>{
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into()
        ));
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


fn main() {

    let mut sys = System::new();

    let zk_urls = zk_server_urls();
    println!("connecting to {}", zk_urls);
    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15),
                                LoggingWatcher).unwrap();


    let ctx = zmq::Context::new();

    let mut zmq_socket = ctx.socket(zmq::STREAM).unwrap();
    zmq_socket.connect("tcp://localhost:11234").unwrap();

    // connection to zookeeper
    let mut zk_conn = zk::Server::new();
    zk_conn.zmq_host = match env::var("ZMQ_HOST") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(),
    };

    zk_conn.zmq_tcp_port = match env::var("ZMQ_HOST") {
        Ok(val) => val,
        Err(_) => "11234".to_string(),
    };

//    protobuf::parse_from_bytes()
    let mut server_data: Vec<u8> = Vec::from(zk_conn.write_to_bytes().unwrap());

    // call once
    if let Err(e) = zk.create(ZKDIR, Vec::new(), Acl::open_unsafe().clone(), CreateMode::Persistent) {
        println!("{:?}", e);
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
//                println!("{} {}", disk.get_total_space(), disk.get_available_space());
                if disk.get_total_space() > total_space {
                    total_space = disk.get_total_space();
                    available_space = disk.get_available_space();
                }
            }
            zk_conn.available_space = available_space;
            zk_conn.total_space = total_space;
            // send to zk
            let mut server_data: Vec<u8> = Vec::from(zk_conn.write_to_bytes().unwrap());
            zk.set_data(&node, server_data, None);
        }
    });

    rocket::ignite().mount("/", routes![handle_multi]).launch();
}
