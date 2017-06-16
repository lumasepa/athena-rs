use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct KafkaHost {
    host: Ipv4Addr,
    port: u32
}

#[derive(Debug)]
pub struct KafkaBroker {
    hosts: Vec<KafkaHost>,
    name: String
}