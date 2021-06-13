use super::args::Args;

pub mod connection;

pub trait Prober {
    fn probe_ports(&self, args: &Args);
}
