extern crate num_cpus;
extern crate threadpool;

use std::net::SocketAddr;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::time::Duration;

use threadpool::ThreadPool;

use super::Prober;
use super::super::args::Args;

pub struct Connection;

impl Prober for Connection {
    fn probe_ports(&self, args: &Args) {
        let n_workers = num_cpus::get();
        let n_jobs = args.target.len() * args.ports.len();
        let pool = ThreadPool::new(n_workers);

        let (tx, rx) = channel();
        for ip in args.target.iter() {
            for port in args.ports.iter() {
                let ip = ip.clone();
                let port = port.clone();

                let tx = tx.clone();
                pool.execute(move || {
                    let saddr = SocketAddr::new(ip, port);
                    if let Ok(_) = TcpStream::connect(saddr) {
                        println!("{}:{} is open", ip, port);
                    }
                    tx.send(1).unwrap();
                });
            }
        }

        assert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), n_jobs);
    }
}