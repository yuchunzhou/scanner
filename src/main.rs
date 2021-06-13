extern crate jemallocator;

use crate::prober::Prober;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod args;
mod prober;

fn main() {
    let args = args::parse_args().unwrap();
    match args.method.as_str() {
        "connection" => {
            use prober::connection;
            let connection_prober = connection::Connection;
            connection_prober.probe_ports(&args);
        }
        "sync" => {}
        _ => {}
    }
}
