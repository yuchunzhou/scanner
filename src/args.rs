extern crate clap;
extern crate trust_dns_resolver;

use std::net::IpAddr;
use std::process::exit;

#[derive(Debug)]
pub struct Args {
    pub target: Vec<IpAddr>,
    pub ports: Vec<u16>,
    pub method: String,
}

impl Args {
    fn new() -> Self {
        Args {
            target: vec![],
            ports: vec![],
            method: "".to_string(),
        }
    }
}

#[inline]
fn dns_resolve(host: &str) -> Vec<IpAddr> {
    let resolver = trust_dns_resolver::Resolver::new(
        trust_dns_resolver::config::ResolverConfig::default(),
        trust_dns_resolver::config::ResolverOpts::default(),
    ).unwrap();

    let ips = match resolver.lookup_ip(host) {
        Ok(ips) => ips,
        Err(_) => {
            eprintln!("resolve host {:?} failed!", host);
            exit(1);
        }
    };

    let mut addrs: Vec<IpAddr> = Vec::new();
    for ip in ips.iter() {
        addrs.push(ip.clone());
    }

    assert_ne!(addrs.len(), 0 as usize);
    addrs
}

pub fn parse_args() -> Args {
    let matches = clap::App::new("Port Scanner")
        .version("0.1.0\n")
        .author("yuchunzhou chunzhou.yu@qq.com")
        .about("a network port scanner")
        .arg(
            clap::Arg::with_name("target")
                .short("t")
                .long("target")
                .help("Target host")
                // .required(true)
                .takes_value(true)
                .display_order(0),
        )
        .arg(
            clap::Arg::with_name("ports")
                .short("p")
                .long("ports")
                .help("Port list")
                .multiple(true)
                .takes_value(true)
                .display_order(1),
        )
        .arg(
            clap::Arg::with_name("all")
                .short("a")
                .long("all")
                .help("Whether to scan all ports")
                .takes_value(true)
                .possible_values(&["true", "false"])
                .default_value("false")
                .display_order(2)
        )
        .arg(
            clap::Arg::with_name("method")
                .short("m")
                .long("method")
                .help("Scan method")
                .takes_value(true)
                .possible_values(&["connection", "sync"])
                .default_value("connection")
                .display_order(3)
        )
        .get_matches();

    let mut args = Args::new();
    let target = matches.value_of("target").unwrap();
    args.target = dns_resolve(target);

    if matches.is_present("ports") {
        let port_list = match matches.values_of("ports") {
            Some(ports) => {
                let mut port_list = Vec::new();
                for port in ports.into_iter() {
                    match port.parse::<u16>() {
                        Ok(p) => port_list.push(p),
                        Err(e) => {
                            eprintln!("parse port {:?} failed: {}!", port, e.to_string());
                            exit(1);
                        }
                    }
                }
                port_list
            }
            None => vec![]
        };
        args.ports = port_list;
    }

    if matches.is_present("all") && matches.value_of("all").unwrap() == "true" {
        args.ports = (1..=65535).collect();
    }

    if args.ports.len() == 0 {
        println!("you must choose to scan all ports or some ports!");
        exit(0);
    }

    if matches.is_present("method") {
        args.method = matches.value_of("method").unwrap().to_string();
    } else {
        args.method = "connection".to_string();
    }

    #[cfg(feature = "debug")]
    println!("{:?}", args);

    args
}