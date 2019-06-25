#![feature(proc_macro_hygiene, decl_macro)]

#![deny(unused_mut)]
extern crate zookeeper;
#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use] extern crate rocket;


//extern crate zmq;

pub mod db;

pub mod zk;

pub use zk::{Server, file_descriptor_proto};

pub mod server;

//mod zip;