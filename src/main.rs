use std::io;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::env;
use std::sync::mpsc;
use std::str::{self, from_utf8};

use zookeeper::{Acl, CreateMode, Watcher, WatchedEvent, ZooKeeper};
use zookeeper::recipes::cache::PathChildrenCache;
use protobuf::{self, Message, ProtobufResult};

mod zk;

use zk::Server;



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


fn main() {
    let zk_urls = zk_server_urls();
    println!("connecting to {}", zk_urls);
    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15),
                                LoggingWatcher).unwrap();


    let ctx = zmq::Context::new();

    let mut zmq_socket = ctx.socket(zmq::STREAM).unwrap();
    zmq_socket.connect("tcp://localhost:11234").unwrap();

    let mut server = zk::Server::new();
    server.zmq_host = match env::var("ZMQ_HOST") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(),
    };

    server.zmq_tcp_port = match env::var("ZMQ_HOST") {
        Ok(val) => val,
        Err(_) => "11234".to_string(),
    };
//    protobuf::parse_from_bytes()
    let mut server_data: Vec<u8> = Vec::from(server.write_to_bytes().unwrap());
//    server.write_to_with_cached_sizes(&mut server_data);

    let path = zk.create("/fs/Node",
                         server_data,
                         Acl::open_unsafe().clone(),
                         CreateMode::EphemeralSequential);

    println!("created -> {:?}", path);

    loop {

        let data = zmq_socket.recv_multipart(0).unwrap();
        println!(
            "Identity: {:?} Message : {}",
            data[0],
            str::from_utf8(&data[1]).unwrap()
        );
    }


}
