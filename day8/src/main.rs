extern crate anymap;
use std::net::Ipv4Addr;
use anymap::AnyMap;

#[derive(Debug)]
enum HostAddress {
    DomainName(String),
    Ip(Ipv4Addr),
}

#[derive(Debug)]
struct Port(u32);


#[derive(Debug)]
struct ConnectionLimit(u32);


fn main() {
    let mut config = AnyMap::new();
    config.insert(HostAddress::DomainName("localhost".to_string()));
    config.insert(Port(80));
    config.insert(ConnectionLimit(32));

    println!("{:?}", config.get::<HostAddress>());
    println!("{:?}", config.get::<Port>());
    assert!(config.get::<String>().is_none());
    assert!(config.get::<u32>().is_none());
}
