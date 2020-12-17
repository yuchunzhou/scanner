use clap;
use std::net;
use std::thread;
use std::time;
use trust_dns_resolver;

fn dns_resolve(host: &str) -> Vec<net::IpAddr> {
    let resolver = trust_dns_resolver::Resolver::new(
        trust_dns_resolver::config::ResolverConfig::default(),
        trust_dns_resolver::config::ResolverOpts::default(),
    )
    .unwrap();
    let response = match resolver.lookup_ip(host) {
        Ok(result) => result,
        Err(err) => {
            println!("{:?}", err);
            std::process::exit(1);
        }
    };

    let mut addrs: Vec<net::IpAddr> = Vec::new();
    for addr in response.iter() {
        addrs.push(addr.clone());
    }

    assert_ne!(addrs.len(), 0 as usize);
    addrs
}

fn probe_ports(ip_list: Vec<net::IpAddr>, port_list: Vec<u16>) {
    let mut handlers: Vec<thread::JoinHandle<_>> = Vec::new();

    for ip in ip_list.iter() {
        for port in port_list.iter() {
            let ip = ip.clone();
            let port = port.clone();

            let handler = thread::spawn(move || {
                let saddr = net::SocketAddr::new(ip, port);
                if let Ok(_) = net::TcpStream::connect_timeout(&saddr, time::Duration::from_secs(1))
                {
                    println!("{}:{} is open", ip, port);
                } else {
                    println!("{}:{} is closed", ip, port);
                }
            });
            handlers.push(handler);
        }
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

fn main() {
    let matches = clap::App::new("Port Scanner")
        .version("1.0\n")
        .author("yuchunzhou chunzhou.yu@qq.com")
        .about("a network port scanner")
        .arg(
            clap::Arg::with_name("target")
                .short('t')
                .long("target")
                .about("target host")
                .takes_value(true)
                .display_order(0),
        )
        .arg(
            clap::Arg::with_name("ports")
                .short('p')
                .long("ports")
                .about("port list")
                .multiple(true)
                .takes_value(true)
                .display_order(1),
        )
        .get_matches();

    let target = match matches.value_of("target") {
        Some(target) => target,
        None => return,
    };

    let ip_list = match target.parse::<net::IpAddr>() {
        Ok(ip) => vec![ip],
        Err(_) => dns_resolve(&target),
    };

    let port_list = match matches.values_of("ports") {
        Some(ports) => {
            let mut port_list = Vec::new();
            for port in ports.into_iter() {
                match port.parse::<u16>() {
                    Ok(port) => port_list.push(port),
                    Err(_) => {
                        println!("parse port {:?} failed!", port);
                        continue;
                    }
                }
            }
            port_list
        }
        None => return,
    };

    probe_ports(ip_list, port_list);
}
