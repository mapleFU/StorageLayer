
const ZKDIR: &'static str = "/fs";

/// Config is server config
pub struct Config {
    /// http config
    http_serv_addr: String,
    /// zmq config
    zmq_host: String,
    zmq_http_port: String,

    /// zk config
    zk_bind: String,
    zk_dir: String,
    data_dir: String
}

pub struct Server {

}